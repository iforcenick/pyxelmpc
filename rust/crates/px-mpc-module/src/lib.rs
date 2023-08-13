use futures::channel::mpsc;
use futures::executor::block_on;
use futures::{SinkExt, StreamExt, TryStreamExt};

use anyhow::{anyhow, Context, Result};
use libc::{c_char, c_int};

use px_mpc_protocol::gg20::state_machine::keygen::Keygen;

use px_round_based::{
    async_runtime::AsyncProtocol,
    Msg
};

mod gg20_sm_client;
use gg20_sm_client::{generate_outgoing_message, join_computation};

use px_mpc_protocol::gg20::state_machine::sign::{
    OfflineStage, SignManual, CompletedOfflineStage,
};

use std::ffi::CString;
use std::thread;

use curv::arithmetic::Converter;
use curv::BigInt;

pub struct Channel {
    sender: Box<mpsc::Sender<String>>,
    receiver: Box<mpsc::Receiver<String>>,
}

#[no_mangle]
pub extern "C" fn create_channel() -> *mut std::ffi::c_void {
    let (sender, receiver) = mpsc::channel::<String>(50);

    let channel = Channel {
        sender: Box::new(sender),
        receiver: Box::new(receiver),
    };

    Box::into_raw(Box::new(channel)) as *mut std::ffi::c_void
}

#[no_mangle]
pub extern "C" fn dispatch_incoming(
    channel: *mut std::ffi::c_void,
    message: *const c_char,
) -> *mut std::ffi::c_void {
    let channel = unsafe { Box::from_raw(channel as *mut Channel) };
    let mut tx = channel.sender.clone();
    let message_str: String = unsafe {
        std::ffi::CStr::from_ptr(message)
            .to_string_lossy()
            .into_owned()
    };

    // Convert the string into a message and send it through the channel
    let _ = tx.try_send(message_str).unwrap();
    // tx.try_send(message_str).unwrap();

    // Reclaim memory allocated by Box::from_raw
    // std::mem::forget(tx);
    Box::into_raw(channel) as *mut std::ffi::c_void
}







#[tokio::main]
async fn _generate_key_async(
    index: u16,
    unique_id: u16,
    incoming_receiver: Box<mpsc::Receiver<String>>,
    callback: unsafe extern "C" fn(*const c_char, c_int),
) -> Result<()> {
    let threshold = 1;
    let number_of_parties = 3;

    let (incoming, outgoing) = join_computation(unique_id, incoming_receiver, callback)
        .await
        .context("join computation")?;

    let incoming = incoming.fuse();
    tokio::pin!(incoming);
    tokio::pin!(outgoing);

    let keygen = Keygen::new(index, threshold, number_of_parties)?;
    let output = AsyncProtocol::new(keygen, incoming, outgoing)
        .run()
        .await
        .map_err(|e| anyhow!("protocol execution terminated with error: {}", e))?;
    let output = serde_json::to_string(&output).context("serialize output")?;
    let outgoing = generate_outgoing_message("key", &output);
    let outgoing = CString::new(outgoing).unwrap();
    unsafe { callback(outgoing.as_ptr(), outgoing.as_bytes().len() as c_int) };

    Ok(())
}

#[no_mangle]
pub extern "C" fn generate_key(
    channel: *mut std::ffi::c_void,
    index: u16,
    unique_id: u16,
    callback: unsafe extern "C" fn(*const c_char, c_int),
) -> *mut std::ffi::c_void {
    let mut channel = unsafe { Box::from_raw(channel as *mut Channel) };
    let receiver = channel.receiver;
    thread::spawn(move || {
        _generate_key_async(index, unique_id, receiver, callback);
    });
    let (_, temp_receiver) = mpsc::channel::<String>(50);
    channel.receiver = Box::new(temp_receiver);

    Box::into_raw(channel) as *mut std::ffi::c_void
}









#[tokio::main]
async fn _create_offline_stage_async(
    unique_id: u16,
    local_share: String,
    parties: Vec<u16>,
    incoming_receiver: Box<mpsc::Receiver<String>>,
    callback: unsafe extern "C" fn(*const c_char, c_int),
) -> Result<()> {
    let local_share = serde_json::from_str(&local_share).context("parse local share")?;

    let (incoming, outgoing) = join_computation(unique_id, incoming_receiver, callback)
        .await
        .context("join computation")?;

    let incoming = incoming.fuse();
    tokio::pin!(incoming);
    tokio::pin!(outgoing);

    let signing = OfflineStage::new(unique_id, parties, local_share)?;
    let completed_offline_stage = AsyncProtocol::new(signing, incoming, outgoing)
        .run()
        .await
        .map_err(|e| anyhow!("protocol execution terminated with error: {}", e))?;

    let output = serde_json::to_string(&completed_offline_stage).context("serialize output")?;
    let outgoing_msg = generate_outgoing_message("stage", &output);
    let outgoing_msg = CString::new(outgoing_msg).unwrap();
    unsafe { callback(outgoing_msg.as_ptr(), outgoing_msg.as_bytes().len() as c_int) };

    Ok(())
}

#[no_mangle]
pub extern "C" fn create_offline_stage(
    channel: *mut std::ffi::c_void,
    unique_id: u16,
    local_share: *const c_char,
    parties: *const c_char,
    callback: unsafe extern "C" fn(*const c_char, c_int),
) -> *mut std::ffi::c_void {
    let mut channel = unsafe { Box::from_raw(channel as *mut Channel) };
    let receiver = channel.receiver;
    let local_share: String = unsafe {
        std::ffi::CStr::from_ptr(local_share)
            .to_string_lossy()
            .into_owned()
    };

    let parties: String = unsafe {
        std::ffi::CStr::from_ptr(parties)
            .to_string_lossy()
            .into_owned()
    };

    let parties: Vec<u16> = parties.split(",").map(|party| {
        party.parse::<u16>().unwrap()
    }).collect();

    let parties = vec![parties[0], parties[1]];
    thread::spawn(move || {
        _create_offline_stage_async(unique_id, local_share, parties, receiver, callback);
    });
    let (_, temp_receiver) = mpsc::channel::<String>(50);
    channel.receiver = Box::new(temp_receiver);

    Box::into_raw(channel) as *mut std::ffi::c_void
}








#[tokio::main]
async fn _create_signature_async(
    index: u16,
    unique_id: u16,
    completed_offline_stage: String,
    data_to_sign: String,
    incoming_receiver: Box<mpsc::Receiver<String>>,
    callback: unsafe extern "C" fn(*const c_char, c_int),
) -> Result<()> {
    let number_of_parties = 2;

    let completed_offline_stage: CompletedOfflineStage = serde_json::from_str(&completed_offline_stage).context("parse local share")?;

    let (incoming, outgoing) = join_computation(unique_id, incoming_receiver, callback)
        .await
        .context("join online computation")?;

    tokio::pin!(incoming);
    tokio::pin!(outgoing);

    let (signing, partial_signature) = SignManual::new(
        BigInt::from_bytes(data_to_sign.as_bytes()),
        completed_offline_stage.clone(),
    )?;

    let partial_signature_output = partial_signature.clone();
    let output = serde_json::to_string(&partial_signature_output).context("serialize output")?;
    let outgoing_msg = generate_outgoing_message("partial_signature", &output);
    let outgoing_msg = CString::new(outgoing_msg).unwrap();
    unsafe { callback(outgoing_msg.as_ptr(), outgoing_msg.as_bytes().len() as c_int) };

    outgoing
        .send(Msg {
            sender: index,
            receiver: None,
            body: partial_signature,
        })
        .await?;

    let partial_signatures: Vec<_> = incoming
        .take(number_of_parties - 1)
        .map_ok(|msg| msg.body)
        .try_collect()
        .await?;
    let signature = signing
        .complete(&partial_signatures)
        .context("online stage failed")?;

    let pubkey = completed_offline_stage.public_key();
    let pubkey_str = serde_json::to_string(&pubkey).context("")?;
    let outgoing_msg = generate_outgoing_message("public_key", &pubkey_str);
    let outgoing_msg = CString::new(outgoing_msg).unwrap();
    unsafe { callback(outgoing_msg.as_ptr(), outgoing_msg.as_bytes().len() as c_int) };

    let signature = serde_json::to_string(&signature).context("serialize signature")?;
    let outgoing_msg = generate_outgoing_message("signature", &signature);
    let outgoing_msg = CString::new(outgoing_msg).unwrap();
    unsafe { callback(outgoing_msg.as_ptr(), outgoing_msg.as_bytes().len() as c_int) };

    Ok(())
}

#[no_mangle]
pub extern "C" fn create_signature(
    channel: *mut std::ffi::c_void,
    index: u16,
    unique_id: u16,
    offline_stage: *const c_char,
    data_to_sign: *const c_char,
    callback: unsafe extern "C" fn(*const c_char, c_int),
) -> *mut std::ffi::c_void {
    let mut channel = unsafe { Box::from_raw(channel as *mut Channel) };
    let receiver = channel.receiver;

    let completed_offline_stage: String = unsafe {
        std::ffi::CStr::from_ptr(offline_stage)
            .to_string_lossy()
            .into_owned()
    };

    let data_to_sign: String = unsafe {
        std::ffi::CStr::from_ptr(data_to_sign)
            .to_string_lossy()
            .into_owned()
    };

    thread::spawn(move || {
        _create_signature_async(index, unique_id, completed_offline_stage, data_to_sign, receiver, callback);
    });
    let (_, temp_receiver) = mpsc::channel::<String>(50);
    channel.receiver = Box::new(temp_receiver);

    Box::into_raw(channel) as *mut std::ffi::c_void
}

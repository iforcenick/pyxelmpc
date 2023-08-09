// use libc::{c_char, c_int};
// use std::ffi::CString;
// use std::os::raw::c_void;
// use std::os::raw::c_ulong;
// use std::thread;
// use std::time::Duration;
// use std::ptr::null_mut;
// use std::slice;

// #[no_mangle]
// pub extern "C" fn start_timer(callback: unsafe extern "C" fn(*const c_char, c_int)) {
//     thread::spawn(move || {
//         let msg = CString::new("Hello from Rust").unwrap();
//         loop {
//             unsafe { callback(msg.as_ptr(), msg.as_bytes().len() as c_int) };
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
// }

use libc::{c_char, c_int};
use std::ffi::CString;

use anyhow::{anyhow, Context, Result};
use futures::StreamExt;

use px_mpc_protocol::gg20::state_machine::keygen::Keygen;
use px_round_based::async_runtime::AsyncProtocol;

mod gg20_sm_client;
use gg20_sm_client::join_computation;

#[tokio::main]
async fn _generate_key_async(callback: unsafe extern "C" fn(*const c_char, c_int)) -> Result<()> {
    let address: surf::Url = surf::Url::parse("http://localhost:8000/")?;
    let room: String = "default-keygen".to_string();
    let index = 1;
    let threshold = 1;
    let number_of_parties = 3;

    let (_i, incoming, outgoing) = join_computation(address, &room)
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
    let output = serde_json::to_vec_pretty(&output).context("serialize output")?;
    
    let msg = CString::new(output).unwrap();
    unsafe { callback(msg.as_ptr(), msg.as_bytes().len() as c_int) };

    Ok(())
}

#[no_mangle]
pub extern "C" fn generate_key(callback: unsafe extern "C" fn(*const c_char, c_int)) {
    _generate_key_async(callback);
}
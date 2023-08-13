use anyhow::{Context, Result};
use futures::{Sink, Stream, StreamExt, TryStreamExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use px_round_based::Msg;
use futures::channel::mpsc;

use libc::{c_char, c_int};
use std::ffi::CString;

pub async fn join_computation<M>(
    index: u16,
    incoming_receiver: Box<mpsc::Receiver<String>>,
    callback: unsafe extern "C" fn(*const c_char, c_int),
) -> Result<(
    impl Stream<Item = Result<Msg<M>>>,
    impl Sink<Msg<M>, Error = anyhow::Error>,
)>
where
    M: Serialize + DeserializeOwned,
{
    let client = SmClient::new(callback).context("construct SmClient")?;

    // Construct channel of incoming messages
    let incoming = client
        .subscribe(incoming_receiver)
        .await
        .context("subscribe")?
        .and_then(|msg| async move {
            serde_json::from_str::<Msg<M>>(&msg).context("deserialize message")
        });

    // Ignore incoming messages addressed to someone else
    let incoming = incoming.try_filter(move |msg| {
        futures::future::ready(
            msg.sender != index && (msg.receiver.is_none() || msg.receiver == Some(index)),
        )
    });

    // Construct channel of outgoing messages
    let outgoing = futures::sink::unfold(client, |client, message: Msg<M>| async move {
        let serialized = serde_json::to_string(&message).context("serialize message")?;
        client
            .broadcast(&serialized)
            .await
            .context("broadcast message")?;
        Ok::<_, anyhow::Error>(client)
    });

    Ok((incoming, outgoing))
}

pub struct SmClient {
    callback: unsafe extern "C" fn(*const c_char, c_int),
}

impl SmClient {
    pub fn new(
        callback: unsafe extern "C" fn(*const c_char, c_int),
    ) -> Result<Self> {
        Ok(Self {
            callback: callback,
        })
    }

    pub async fn broadcast(&self, message: &str) -> Result<()> {
        let outgoing_message = generate_outgoing_message("broadcast", message);
        let msg = CString::new(outgoing_message).unwrap();
        unsafe { (self.callback)(msg.as_ptr(), msg.as_bytes().len() as c_int) };
        Ok(())
    }

    pub async fn subscribe(&self, incoming_receiver: Box<mpsc::Receiver<String>>) -> Result<impl Stream<Item = Result<String>>> {
        Ok(incoming_receiver.filter_map(|msg| async {
            Some(Ok(msg))
        }))
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OutgoingMessage<'a, 'b> {
    pub data_type: &'a str,
    pub data: &'b str,
}

pub fn generate_outgoing_message<'a, 'b>(data_type: &'a str, data: &'b str) -> String {
    let outgoing_message = OutgoingMessage { data_type, data };
    return serde_json::to_string(&outgoing_message).unwrap();
}
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use futures::{SinkExt, StreamExt, TryStreamExt};
use structopt::StructOpt;

use curv::arithmetic::Converter;
use curv::BigInt;

use px_mpc_protocol::gg20::state_machine::sign::{
    OfflineStage, SignManual, CompletedOfflineStage,
};
use px_round_based::{
    async_runtime::AsyncProtocol,
    Msg
};

mod gg20_sm_client;
use gg20_sm_client::join_computation;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "http://localhost:8000/")]
    address: surf::Url,
    #[structopt(short, long, default_value = "default-signing")]
    room: String,
    #[structopt(short, long)]
    offline_stage: PathBuf,

    #[structopt(short, long, use_delimiter(true))]
    parties: Vec<u16>,
    #[structopt(short, long)]
    data_to_sign: String,
    #[structopt(short, long)]
    unique_id: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Cli = Cli::from_args();
    let offline_stage = tokio::fs::read(args.offline_stage)
        .await
        .context("cannot read local share")?;
    let completed_offline_stage: CompletedOfflineStage = serde_json::from_slice(&offline_stage).context("parse local share")?;
    let number_of_parties = args.parties.len();

    let (_i, incoming, outgoing) = join_computation(args.address, &format!("{}-online", args.room))
        .await
        .context("join online computation")?;

    tokio::pin!(incoming);
    tokio::pin!(outgoing);

    let (signing, partial_signature) = SignManual::new(
        BigInt::from_bytes(args.data_to_sign.as_bytes()),
        completed_offline_stage.clone(),
    )?;

    outgoing
        .send(Msg {
            sender: args.unique_id,
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
    let signature = serde_json::to_string(&signature).context("serialize signature")?;
    println!("{}", signature);
    let pubkey = completed_offline_stage.public_key();
    let pubkey_str = serde_json::to_string(&pubkey).context("")?;
    println!("Pubkey: {}", pubkey_str);

    Ok(())
}

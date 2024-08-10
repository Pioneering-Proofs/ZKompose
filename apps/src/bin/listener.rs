use alloy::{
    primitives::{address, Address},
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::{
        self,
        types::{BlockNumberOrTag, Filter},
    },
    sol,
    sol_types::SolEvent,
};
use clap::Parser;
use ethers::providers::StreamExt;
use std::env::var;

sol! {
    interface IPlayers {
        enum Tier {
            Diamond,
            Platinum,
            Gold,
            Silver,
            Bronze
        }

        struct Secp256k1PubKey {
            uint256 x;
            uint256 y;
        }
        event PackRequested(address indexed requester, uint256 indexed packId, Tier indexed tier, Secp256k1PubKey key);
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// WS RPS URL
    #[clap(long, env = "WS_RPC_PROVIDER")]
    rpc_url: String,

    /// Deployed player contract address
    #[clap(long, env = "PLAYER_CONTRACT")]
    player_contract: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let player_contract = address!("74BF0B8F065AA2627EA25c6A07CBA79407Ae7265");
    let rpc_provider = {
        let ws = WsConnect::new(args.rpc_url);
        ProviderBuilder::new().on_ws(ws)
    };
    let rpc_provider = rpc_provider.await.expect("Failed to connect");

    let filter = Filter::new()
        .address(player_contract)
        .event(IPlayers::PackRequested::SIGNATURE)
        .from_block(BlockNumberOrTag::Latest);

    let sub = rpc_provider
        .subscribe_logs(&filter)
        .await
        .expect("Failed to subscribe to logs");
    let mut stream = sub.into_stream();

    println!(
        "Listening Pack Request events on contract {}",
        player_contract
    );

    while let Some(log) = stream.next().await {
        println!("Received log: {:?}", log);

        // TODO: Call generate pack guest code
    }
}

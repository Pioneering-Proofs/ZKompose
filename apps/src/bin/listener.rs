use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{address, fixed_bytes, Address, FixedBytes, U256},
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter, TransactionRequest},
    signers::local::PrivateKeySigner,
    sol,
    sol_types::{SolEvent, SolInterface, SolValue},
};
use clap::Parser;
use common::{
    math::new_u_v,
    types::{GenPlayersJournal, Player},
    utils::match_player_tier,
};
use ethers::providers::StreamExt;
use methods::GEN_PLAYER_ELF;
use pinata_sdk::{PinByFile, PinByJson, PinataApi};
use rand::{thread_rng, Rng};
use risc0_ethereum_contracts::groth16;
use risc0_zkvm::{default_prover, serde, ExecutorEnv, Output, ProverOpts, VerifierContext};
use tokio::task::spawn_blocking;

sol! {
    interface IPlayers {
        enum Tier {
            Diamond,
            Platinum,
            Gold,
            Silver,
            Bronze
        }

        event PackRequested(address indexed requester, uint256 indexed packId, Tier indexed tier);
        function fulfillPackOrder(uint256 orderId, bytes32[15] calldata URIs, bytes calldata seal) external;
    }

    struct Input {
        uint8 tier;
        uint256 order_id;
        uint8 std_dev;
        uint256 u;
        uint256 v;
    }

    struct Journal {
        uint8 tier;
        uint256 order_id;
        bytes32[15] cids;
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// WS RPC URL
    #[clap(long, env = "WS_RPC_PROVIDER")]
    ws_url: String,

    /// Deployed player contract address
    #[clap(long, env = "PLAYER_CONTRACT")]
    player_contract: Option<String>,

    /// Deployed player contract address
    #[clap(long, env = "PRIV_KEY")]
    priv_key: String,
}

async fn request_proof(input: Vec<u8>) -> Option<(Journal, Vec<u8>)> {
    let env = ExecutorEnv::builder()
        .write_slice(&input)
        // .unwrap()
        .build()
        .unwrap();

    let receipt = default_prover()
        .prove_with_ctx(
            env,
            &VerifierContext::default(),
            GEN_PLAYER_ELF,
            &ProverOpts::groth16(),
        )
        .unwrap()
        .receipt;
    let seal = groth16::encode(receipt.inner.groth16().ok()?.seal.clone()).unwrap();
    let journal = receipt.journal.bytes.clone();
    let input = Input::abi_decode(&input, true).expect("Failed to decode input");

    let output: Journal = <Journal>::abi_decode(&journal, true).expect("Failed to decode output");
    let cids = output.cids.clone();

    let median = match_player_tier(input.tier);
    let mut u = input.u.to::<u32>() as f64;
    let mut v = input.v.to::<u32>() as f64;
    let pinata_api_key = std::env::var("PINATA_API_KEY").unwrap();
    let pinata_secret_api_key = std::env::var("PINATA_SECRET_API_KEY").unwrap();
    let pinata = PinataApi::new(pinata_api_key, pinata_secret_api_key).unwrap();

    for i in 0..15 {
        (u, v) = new_u_v(u, v);
        let player = Player::new(
            (input.order_id.to::<u32>() * 15) + i as u32,
            input.std_dev,
            median,
            u,
            v,
        );

        // if player.cid() != cids[i] {
        //     return None;
        // }

        let result = pinata
            .pin_json(PinByJson::new(player.to_json_string()))
            .await;
        if let Ok(pinned_object) = result {
            let hash = pinned_object.ipfs_hash;
        }
    }
    println!("Generated players: {:?}", output.cids.len());

    Some((output, seal))
}

fn match_bytes(tier: FixedBytes<32>) -> u8 {
    let tier: U256 = tier.into();
    for i in 0..5 {
        if tier == U256::from(i) {
            return i;
        }
    }
    0
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let player_contract: Address = address!("B59612143d5DE1CFdd0403459B35D8A2CC73164F");

    let signer: PrivateKeySigner = args.priv_key.clone().parse().expect("Invalid private key");
    let wallet = EthereumWallet::from(signer);
    let rpc_provider = {
        let ws = WsConnect::new(args.ws_url);
        ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_ws(ws)
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
        match log.topic0() {
            Some(&IPlayers::PackRequested::SIGNATURE_HASH) => {
                let IPlayers::PackRequested {
                    requester,
                    packId,
                    tier,
                } = log.log_decode().unwrap().inner.data;
                println!("Pack requested by {requester} with packId {packId} of tier {tier}");
                let u: u32 = thread_rng().gen_range(0..100);
                let v: u32 = thread_rng().gen_range(0..5_000);

                let result = spawn_blocking(move || {
                    let proof = request_proof(
                        Input {
                            tier: match_bytes(tier),
                            order_id: U256::from(packId),
                            std_dev: 5,
                            u: U256::from(u),
                            v: U256::from(v),
                        }
                        .abi_encode(),
                    );
                    proof
                });
                let (output, seal) = result.await.unwrap().await.unwrap();

                for cid in output.cids.iter() {
                    println!("CID: {:?}", cid);
                }

                let call: IPlayers::fulfillPackOrderCall = IPlayers::fulfillPackOrderCall {
                    orderId: output.order_id,
                    URIs: output.cids,
                    seal: seal.into(),
                };
                let tx = TransactionRequest::default()
                    .with_to(player_contract)
                    .with_call(&call);

                let tx = rpc_provider.send_transaction(tx).await.unwrap();
                println!("Transaction receipt: {:?}", tx);
            }
            _ => {}
        }

        // TODO: Call generate pack guest code
    }
}

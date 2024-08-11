use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{address, fixed_bytes, Address, FixedBytes},
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter, TransactionRequest},
    signers::local::PrivateKeySigner,
    sol,
    sol_types::{SolEvent, SolInterface, SolValue},
};
use alloy_primitives::U256;
use alloy_sol_types::{sol, SolValue};
use clap::Parser;
use common::types::{GenPlayersInput, GenPlayersJournal};
use ethers::providers::StreamExt;
use methods::GEN_PLAYER_ELF;
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

        struct Secp256k1PubKey {
            bytes[33] key;
        }

        event PackRequested(address indexed requester, uint256 indexed packId, Tier indexed tier, Secp256k1PubKey key);
        function fulfillPackOrder(uint256 orderId, string[15] calldata URIs, bytes calldata seal) external;
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

fn request_proof(input: Vec<u8>) -> Option<(GenPlayersJournal, Vec<u8>)> {
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

    let output: GenPlayersJournal = serde::from_slice(&journal).expect("Failed to decode output");
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

sol! {
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
                    key: _,
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
                let (output, seal) = result.await.unwrap().unwrap();

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

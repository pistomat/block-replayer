use block_replayer::block_replayer::BlockReplayer;

use clap::Parser;
use tracing::info;

const BLOCK_NUMBER: &str = "17181508";
const BLOCKS_TO_REPLAY: &str = "100";

#[derive(Parser)]
#[clap(name = "block-replayer", version)]
/// Example:
/// cargo run -- --fork-url http://localhost:8545 --fork-block-number 17181508 --blocks-to-replay 100
struct Args {
    #[clap(long, short = 'f')]
    fork_url: String,
    #[clap(long, default_value = BLOCK_NUMBER)]
    fork_block_number: u64,
    #[clap(long, default_value = BLOCKS_TO_REPLAY)]
    blocks_to_replay: u64,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let args = Args::parse();

    let replayer = BlockReplayer::new(args.fork_url, args.fork_block_number).await;

    info!("Replaying blocks");
    replayer.replay_blocks(args.blocks_to_replay).await.unwrap();
}

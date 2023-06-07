use block_replayer::block_replayer::BlockReplayer;

use clap::Parser;
use tracing::info;

const BLOCK_NUMBER: &str = "17181508";

#[derive(Parser)]
#[clap(name = "block-replayer", version)]
struct Args {
    #[clap(long, short = 'f')]
    fork_url: String,
    #[clap(long, short, default_value = BLOCK_NUMBER)]
    fork_block_number: u64,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let args = Args::parse();

    let replayer = BlockReplayer::new(args.fork_url, args.fork_block_number).await;

    info!("block_replayer: {}", replayer);
}

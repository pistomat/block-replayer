use std::fmt::{Formatter, self};

use anvil::{eth::EthApi, spawn, NodeConfig, NodeHandle};
use ethers::providers::Http;
use url::Url;

#[allow(dead_code)]
pub struct BlockReplayer {
    origin_provider: Http,
    fork_api: EthApi,
    fork_handle: NodeHandle,
}

impl BlockReplayer {
    pub async fn new(fork_url: String, fork_block_number: u64) -> Self {
        let url = Url::parse(&fork_url).unwrap();
        let origin_provider = Http::new(url);

        let config = NodeConfig::default()
            .with_fork_block_number(Some(fork_block_number))
            .with_eth_rpc_url(Some(fork_url));

        let (fork_api, fork_handle) = spawn(config).await;

        Self { origin_provider, fork_api, fork_handle }
    }
}

impl fmt::Display for BlockReplayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockReplayer")
            .field("origin_provider", &self.origin_provider)
            .finish()
    }
}

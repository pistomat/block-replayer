use std::fmt::{self, Formatter};

use anvil::{eth::EthApi, spawn, NodeConfig, NodeHandle};
use ethers::{
    providers::{Http, Middleware, Provider, ProviderError},
    types::{Block, BlockNumber, Transaction, U256, U64},
};
use tracing::info;
use url::Url;

#[allow(dead_code)]
pub struct BlockReplayer {
    origin_provider: Provider<Http>,
    fork_api: EthApi,
    fork_handle: NodeHandle,
}

impl BlockReplayer {
    pub async fn new(fork_url: String, fork_block_number: u64) -> Self {
        let url = Url::parse(&fork_url).unwrap();
        let origin_http = Http::new(url);
        let origin_provider = Provider::new(origin_http);

        let config = NodeConfig::default()
            .with_fork_block_number(Some(fork_block_number))
            .with_eth_rpc_url(Some(fork_url))
            .with_no_mining(true);

        let (fork_api, fork_handle) = spawn(config).await;

        Self { origin_provider, fork_api, fork_handle }
    }

    async fn replay_block(
        &self,
        block: Block<Transaction>,
        interval: U256,
    ) -> Result<(), ProviderError> {
        for tx in block.transactions {
            info!("Replaying tx with position {:?}", tx.transaction_index);
            self.fork_api.send_raw_transaction(tx.rlp()).await.unwrap();
        }
        self.fork_api.anvil_mine(Some(U256::from(1)), None).await.unwrap();

        Ok(())
    }

    pub async fn replay_blocks(&self, max_block_increase: u64) -> Result<(), ProviderError> {
        for _ in 0..max_block_increase {
            let fork_latest_block =
                self.fork_api.block_by_number(BlockNumber::Latest).await.unwrap().unwrap();
            let fork_latest_block_number = U64::from(fork_latest_block.number.unwrap().as_u64());
            let fork_latest_block_timestamp = fork_latest_block.timestamp;
            let origin_latest_block_number = self.origin_provider.get_block_number().await.unwrap();

            if origin_latest_block_number == fork_latest_block_number + 1 {
                info!("Fork is caught up to origin, exiting.");
                break
            }
            let block_number_to_replay = BlockNumber::Number(fork_latest_block_number + 1);
            info!("Replaying block {}", block_number_to_replay);

            let block_to_replay = self
                .origin_provider
                .get_block_with_txs(block_number_to_replay)
                .await
                .unwrap()
                .unwrap();
            let interval = block_to_replay.timestamp - fork_latest_block_timestamp;

            self.replay_block(block_to_replay, interval).await?;
        }

        Ok(())
    }
}

impl fmt::Display for BlockReplayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockReplayer").field("origin_provider", &self.origin_provider).finish()
    }
}

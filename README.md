## What is BlockReplayer?
BlockReplayer is a simple tool for replaying historical blocks on a local Ethereum node. It can be used to simulate various scenarios and test different strategies that require a websocket connection to a node.

## Usage
To use BlockReplayer, ensure that you have Rust installed.

Next, clone the github repo:
```
git clone https://github.com/pistomat/block-replayer
cd block-replayer
```

To replay a specific set of blocks, run the following command:

```
cargo run -- --fork-url <ETH_NODE_URL> --fork-block-number <STARTING_BLOCK_NUMBER> --blocks-to-replay <NUMBER_OF_BLOCKS>
```

Replace ```<ETH_NODE_URL>``` with the URL of the Ethereum node from where the blocks will be fetched, ```<STARTING_BLOCK_NUMBER>``` with the block number from where the replaying should start, and ```<NUMBER_OF_BLOCKS>``` with the number of blocks to replay.

The application will then start a new Anvil fork node, fetch the specified blocks from the original node, and replay them on the new node. You can then connect to this node using websockets and use it for testing.
use crate::blockchain::block::Block;
use crate::blockchain::chain::Blockchain;
use crate::blockchain::mempool::Mempool;
use crate::blockchain::transaction::Transaction;
pub use crate::node::node_config::NodeConfig;
use std::cmp::Ordering;
use std::thread::sleep;
use std::time::Duration;

pub struct Node {
    chain: Blockchain,
    pub mempool: Mempool,
    pub node_config: NodeConfig,
    pub logical_time: u64,
}

impl Node {
    pub fn new(
        chain: Blockchain,
        mempool: Mempool,
        node_config: NodeConfig,
        logical_time: u64,
    ) -> Self {
        Self {
            chain,
            mempool,
            node_config,
            logical_time,
        }
    }

    pub fn chain(&self) -> &Blockchain {
        &self.chain
    }

    pub fn submit_transaction(&mut self, tx: Transaction) {
        self.mempool.add_transaction(tx)
    }

    pub fn tick(&mut self) -> Result<(), NodeError> {
        if self.node_config.mining_enabled && self.mempool.len() > 0 {
            println!("Mining...");
            let timestamp = self.logical_time;
            let block = Block::mine(
                &self.chain.tip(),
                self.mempool.peek(),
                self.next_difficulty(),
                timestamp,
            );
            self.chain
                .append_block(block)
                .map_err(|_| NodeError::ChainError)?;
            self.mempool.drain();
            self.logical_time = self.logical_time + 1;
        }
        Ok(())
    }

    pub fn run_tick(&mut self, n_tick: usize) -> Result<(), NodeError> {
        for _ in 1..=n_tick {
            self.tick()?;
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), NodeError> {
        loop {
            println!("Tick...");
            self.run_tick(1)?;
            sleep(Duration::from_secs(self.node_config.tick_interval));
        }
    }

    fn next_difficulty(&self) -> usize {
        let last_block = self.chain.tip();

        if last_block.block_hash == "GENESIS" {
            return self.node_config.difficulty;
        }

        let time_delta = self.logical_time - last_block.timestamp;
        let target_time = self.node_config.block_time_delta;

        match time_delta.cmp(&target_time) {
            Ordering::Greater => last_block.difficulty.saturating_sub(1),
            Ordering::Less => last_block.difficulty + 1,
            Ordering::Equal => last_block.difficulty,
        }
    }
}

#[derive(Debug)]
pub enum NodeError {
    ChainError,
}

use crate::blockchain::block::Block;
use crate::blockchain::chain::Blockchain;
use crate::blockchain::mempool::Mempool;
use crate::blockchain::transaction::Transaction;
use crate::node::node_config::NodeConfig;

pub struct Node {
    chain: Blockchain,
    pub mempool: Mempool,
    pub node_config: NodeConfig,
}

impl Node {
    pub fn new(chain: Blockchain, mempool: Mempool, node_config: NodeConfig) -> Self {
        Self {
            chain,
            mempool,
            node_config,
        }
    }

    pub fn chain(&self) -> &Blockchain {
        &self.chain
    }

    pub fn submit_transaction(&mut self, tx: Transaction) {
        self.mempool.add_transaction(tx)
    }

    pub fn tick(&mut self) -> Result<(), NodeError> {
        if self.mempool.len() > 0 {
            let block = Block::mine(
                &self.chain.tip(),
                self.mempool.peek(),
                self.node_config.difficulty,
            );
            self.chain
                .append_block(block)
                .map_err(|_| NodeError::ChainError)?;
            self.mempool.drain();
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum NodeError {
    ChainError,
}

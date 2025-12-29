use serde::{Deserialize, Serialize};

const GENESIS_BLOCK_HASH: &str = "GENESIS";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub height: u64,
    pub previous_hash: String,
    pub timestamp: u64,
    pub merkle_root: String,
    pub transactions: Vec<String>,
    pub nonce: u64,
    pub block_hash: String,
}

impl Block {
    pub fn genesis() -> Block {
        Block {
            height: 0,
            previous_hash: String::from(GENESIS_BLOCK_HASH),
            timestamp: 0,
            merkle_root: String::from(""),
            transactions: vec![],
            nonce: 0,
            block_hash: String::from(""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_block_has_height_zero() {
        let genesis = Block::genesis();
        assert_eq!(genesis.height, 0, "Genesis block height must be 0");
    }

    #[test]
    fn genesis_block_has_no_transactions() {
        let genesis = Block::genesis();
        assert!(
            genesis.transactions.is_empty(),
            "Genesis block must have no transactions"
        );
    }

    #[test]
    fn genesis_block_has_fixed_previous_hash() {
        let genesis = Block::genesis();
        assert_eq!(
            genesis.previous_hash, "GENESIS",
            "Genesis block previous hash must be 0"
        );
    }

    #[test]
    fn genesis_block_is_deterministic() {
        let a = Block::genesis();
        let b = Block::genesis();

        assert_eq!(
            a.block_hash, b.block_hash,
            "Genesis block must be deterministic"
        );
    }
}

use crate::blockchain::transaction::Transaction;
use crate::crypto::hash::sha256;
use serde::{Deserialize, Serialize};

const GENESIS_BLOCK_HASH: &str = "GENESIS";
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub height: u64,
    pub previous_hash: String,
    // pub timestamp: u64,
    // pub merkle_root: String,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
    pub difficulty: usize,
    pub block_hash: String,
}

impl Block {
    pub fn new(
        height: u64,
        previous_hash: String,
        // timestamp: u64,
        // merkle_root: String,
        transactions: Vec<Transaction>,
        nonce: u64,
        difficulty: usize,
    ) -> Block {
        let block_hash = Self::compute_hash(&height, &previous_hash, &transactions, &nonce);

        Block {
            height,
            previous_hash,
            // timestamp,
            // merkle_root,
            transactions,
            nonce,
            difficulty,
            block_hash,
        }
    }

    pub fn genesis() -> Block {
        Block {
            height: 0,
            previous_hash: String::from(""),
            transactions: vec![],
            nonce: 0,
            difficulty: 0,
            block_hash: String::from(GENESIS_BLOCK_HASH),
        }
    }

    pub fn is_hash_valid(&self) -> bool {
        self.block_hash
            == Self::compute_hash(
                &self.height,
                &self.previous_hash,
                &self.transactions,
                &self.nonce,
            )
    }

    pub fn mine(prev_block: &Block, transactions: Vec<Transaction>, difficulty: usize) -> Block {
        let mut nonce = 0;

        loop {
            let block = Block::new(
                prev_block.height + 1,
                prev_block.block_hash.clone(),
                transactions.clone(),
                nonce,
                difficulty,
            );

            if block.is_pow_valid(difficulty) {
                return block;
            }
            nonce += 1;
        }
    }

    pub fn is_pow_valid(&self, difficulty: usize) -> bool {
        self.block_hash[..difficulty].chars().all(|c| c == '0')
    }

    fn compute_hash(
        height: &u64,
        previous_hash: &String,
        transactions: &Vec<Transaction>,
        nonce: &u64,
    ) -> String {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&height.to_be_bytes());
        bytes.extend_from_slice(previous_hash.as_bytes());

        for tx in transactions {
            bytes.extend_from_slice(tx.tx_id.as_bytes());
            bytes.extend_from_slice(&(tx.payload.len() as u64).to_be_bytes());
            bytes.extend_from_slice(&tx.payload);
        }

        bytes.extend_from_slice(&nonce.to_be_bytes());

        sha256(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tx(payload: &[u8]) -> Transaction {
        Transaction::new(payload.to_vec())
    }

    fn sample_block() -> Block {
        Block::new(1, "prev_hash".to_string(), vec![tx(b"a"), tx(b"b")], 0, 2)
    }

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
            genesis.previous_hash, "",
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

    #[test]
    fn identical_blocks_have_identical_hashes() {
        let a = sample_block();
        let b = sample_block();

        assert_eq!(
            a.block_hash, b.block_hash,
            "Identical blocks must have identical hashes"
        );
    }

    #[test]
    fn changing_previous_hash_changes_block_hash() {
        let block = sample_block();
        let original = block.block_hash.clone();

        let modified = Block::new(
            block.height,
            "modified".to_string(),
            block.transactions.clone(),
            0,
            2,
        );

        assert_ne!(
            original, modified.block_hash,
            "Previous hash must be committed into block hash"
        );
    }

    #[test]
    fn changing_transaction_content_changes_block_hash() {
        let block1 = sample_block();

        let block2 = Block::new(1, "prev_hash".to_string(), vec![tx(b"a"), tx(b"c")], 0, 2);

        assert_ne!(
            block1.block_hash, block2.block_hash,
            "Block hash must commit to transaction content"
        );
    }

    #[test]
    fn changing_transaction_order_changes_block_hash() {
        let block1 = Block::new(1, "prev_hash".to_string(), vec![tx(b"a"), tx(b"b")], 0, 2);

        let block2 = Block::new(1, "prev_hash".to_string(), vec![tx(b"b"), tx(b"a")], 0, 2);

        assert_ne!(
            block1.block_hash, block2.block_hash,
            "Transaction order must affect block hash"
        );
    }
}

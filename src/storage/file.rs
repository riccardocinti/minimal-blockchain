use crate::blockchain::block::Block;
use crate::blockchain::chain::Blockchain;
use serde_json;
use std::fs;

const CHAIN_FILE: &str = "data/chain.json";

pub fn save_chain(chain: &[Block]) {
    let json = serde_json::to_string_pretty(chain).unwrap();
    fs::write(CHAIN_FILE, json).unwrap();
}

pub fn load_chain() -> Blockchain {
    let data = fs::read_to_string(CHAIN_FILE).unwrap_or_else(|_| "[]".to_string());
    Blockchain {
        blocks: serde_json::from_str(&data).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::chain::Blockchain;

    #[test]
    fn persisted_genesis_survives_roundtrip() {
        let chain = Blockchain::init();

        save_chain(&chain.blocks);
        let loaded = load_chain();

        assert_eq!(
            loaded.blocks.len(),
            1,
            "Loaded chain must have exactly one block"
        );
        assert_eq!(
            loaded.blocks[0].block_hash, chain.blocks[0].block_hash,
            "Genesis block hash must survive persistence"
        );
    }
}

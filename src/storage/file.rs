use crate::blockchain::block::Block;
use crate::blockchain::chain::{Blockchain, ChainError};
use serde_json;
use std::fs;
use std::path::Path;

pub const CHAIN_FILE: &str = "data/chain.json";

pub fn save_chain(path: &Path, chain: &[Block]) {
    let json = serde_json::to_string_pretty(chain).unwrap();
    fs::write(path, json).unwrap();
}

pub fn load_chain(path: &Path) -> Result<Blockchain, ChainError> {
    let data = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    match serde_json::from_str(&data) {
        Ok(blocks) => Ok(Blockchain::load(blocks)),
        Err(_) => Err(ChainError::DeserializationError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::chain::Blockchain;

    #[test]
    fn persisted_genesis_survives_roundtrip() {
        let chain = Blockchain::init();

        save_chain(Path::new(CHAIN_FILE), &chain.blocks);
        let loaded = load_chain(Path::new(CHAIN_FILE));

        assert_eq!(
            loaded.as_ref().unwrap().blocks.len(),
            1,
            "Loaded chain must have exactly one block"
        );
        assert_eq!(
            loaded.as_ref().unwrap().blocks[0].block_hash,
            chain.blocks[0].block_hash,
            "Genesis block hash must survive persistence"
        );
    }
}

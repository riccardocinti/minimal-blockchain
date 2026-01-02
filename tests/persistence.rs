use minimal_blockchain::blockchain::{block::Block, chain::Blockchain, transaction::Transaction};

use std::fs;
use std::path::PathBuf;

fn tx(data: &[u8]) -> Transaction {
    Transaction::new(data.to_vec())
}

fn temp_file(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(name);
    path
}

#[test]
fn saved_and_loaded_chain_is_identical() {
    let mut chain = Blockchain::init();

    let b1 = Block::mine(&chain.tip(), vec![tx(b"a")], 2);
    let b2 = Block::mine(&b1, vec![tx(b"b")], 2);

    chain.blocks.push(b1);
    chain.blocks.push(b2);

    let path = temp_file("chain_test.json");

    chain.save_to_file(&path).expect("save failed");
    let loaded = Blockchain::load_from_file(&path).expect("load failed");

    assert_eq!(chain.blocks.len(), loaded.blocks.len());
    assert!(loaded.validate().is_ok());

    let _ = fs::remove_file(path);
}

#[test]
fn loading_invalid_chain_fails() {
    let path = temp_file("invalid_chain.json");

    fs::write(&path, "{ not valid json }").unwrap();

    assert!(Blockchain::load_from_file(&path).is_err());

    let _ = fs::remove_file(path);
}

#[test]
fn loading_tampered_chain_fails_validation() {
    let mut chain = Blockchain::init();

    let mut b1 = Block::new(1, chain.tip().block_hash.clone(), vec![tx(b"a")], 0, 2);
    b1.block_hash = "evil".into();

    chain.blocks.push(b1);

    let path = temp_file("tampered_chain.json");

    chain.save_to_file(&path).unwrap();

    assert!(
        Blockchain::load_from_file(&path).is_err(),
        "Tampered chain must be rejected on load"
    );

    let _ = fs::remove_file(path);
}

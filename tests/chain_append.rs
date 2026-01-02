use minimal_blockchain::blockchain::{block::Block, chain::Blockchain, transaction::Transaction};

fn tx(data: &[u8]) -> Transaction {
    Transaction::new(data.to_vec())
}

#[test]
fn valid_block_is_appended() {
    let mut chain = Blockchain::init();
    let last = chain.blocks.last().unwrap();

    let block = Block::new(
        last.height + 1,
        last.block_hash.clone(),
        vec![tx(b"a"), tx(b"b")],
        0,
        2,
    );

    assert!(
        chain.append_block(block).is_ok(),
        "Valid block should be appended"
    );

    assert_eq!(chain.blocks.len(), 2);
}

#[test]
fn block_with_wrong_height_is_rejected() {
    let mut chain = Blockchain::init();
    let last = chain.blocks.last().unwrap();

    let block = Block::new(
        last.height + 2,
        last.block_hash.clone(),
        vec![tx(b"a")],
        0,
        2,
    );

    assert!(
        matches!(chain.append_block(block), Err(_)),
        "Block with incorrect height must be rejected"
    );
}

#[test]
fn block_with_wrong_previous_hash_is_rejected() {
    let mut chain = Blockchain::init();
    let last = chain.blocks.last().unwrap();

    let block = Block::new(
        last.height + 1,
        "fake_hash".to_string(),
        vec![tx(b"a")],
        0,
        2,
    );

    assert!(
        matches!(chain.append_block(block), Err(_)),
        "Block with incorrect previous hash must be rejected"
    );
}

#[test]
fn tampered_block_hash_is_rejected() {
    let mut chain = Blockchain::init();
    let last = chain.blocks.last().unwrap();

    let mut block = Block::new(
        last.height + 1,
        last.block_hash.clone(),
        vec![tx(b"a")],
        0,
        2,
    );

    // Simulate corruption
    block.block_hash = "evil".to_string();

    assert!(
        matches!(chain.append_block(block), Err(_)),
        "Block with invalid internal hash must be rejected"
    );
}

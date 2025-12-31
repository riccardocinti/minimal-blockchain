use minimal_blockchain::blockchain::{block::Block, chain::Blockchain, transaction::Transaction};

fn tx(data: &[u8]) -> Transaction {
    Transaction::new(data.to_vec())
}

#[test]
fn longer_valid_chain_replaces_current() {
    let mut local = Blockchain::init();

    let b1 = Block::new(1, local.tip().block_hash.clone(), vec![tx(b"a")]);
    local.blocks.push(b1.clone());

    let mut candidate = Blockchain::init();
    candidate.blocks.push(b1);

    let b2 = Block::new(2, candidate.tip().block_hash.clone(), vec![tx(b"b")]);
    candidate.blocks.push(b2);

    let replaced = local.try_replace(candidate).unwrap();

    assert!(replaced);
    assert_eq!(local.tip().height, 2);
}

#[test]
fn shorter_chain_is_rejected() {
    let mut local = Blockchain::init();

    let b1 = Block::new(1, local.tip().block_hash.clone(), vec![tx(b"a")]);
    local.blocks.push(b1.clone());

    let candidate = Blockchain::init(); // shorter

    let replaced = local.try_replace(candidate).unwrap();

    assert!(!replaced);
    assert_eq!(local.tip().height, 1);
}

#[test]
fn invalid_chain_is_rejected_with_error() {
    let mut local = Blockchain::init();

    let mut candidate = Blockchain::init();
    let mut bad = Block::new(1, candidate.tip().block_hash.clone(), vec![tx(b"a")]);
    bad.block_hash = "evil".into();

    candidate.blocks.push(bad);

    assert!(local.try_replace(candidate).is_err());
}

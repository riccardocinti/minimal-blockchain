use minimal_blockchain::blockchain::{block::Block, chain::Blockchain, transaction::Transaction};

fn tx(data: &[u8]) -> Transaction {
    Transaction::new(data.to_vec())
}

#[test]
fn valid_chain_passes_validation() {
    let mut chain = Blockchain::init();

    let b1 = Block::new(1, chain.tip().block_hash.clone(), vec![tx(b"a")]);
    let b2 = Block::new(2, b1.block_hash.clone(), vec![tx(b"b")]);

    chain.blocks.push(b1);
    chain.blocks.push(b2);

    assert!(chain.validate().is_ok());
}

#[test]
fn chain_with_invalid_height_fails() {
    let mut chain = Blockchain::init();

    let b1 = Block::new(2, chain.tip().block_hash.clone(), vec![tx(b"a")]);

    chain.blocks.push(b1);

    assert!(chain.validate().is_err());
}

#[test]
fn chain_with_broken_hash_link_fails() {
    let mut chain = Blockchain::init();

    let b1 = Block::new(1, "fake".into(), vec![tx(b"a")]);

    chain.blocks.push(b1);

    assert!(chain.validate().is_err());
}

#[test]
fn chain_with_tampered_block_fails() {
    let mut chain = Blockchain::init();

    let mut b1 = Block::new(1, chain.tip().block_hash.clone(), vec![tx(b"a")]);
    b1.block_hash = "evil".into();

    chain.blocks.push(b1);

    assert!(chain.validate().is_err());
}

#[test]
fn empty_chain_is_invalid() {
    let chain = Blockchain { blocks: vec![] };

    assert!(chain.validate().is_err());
}

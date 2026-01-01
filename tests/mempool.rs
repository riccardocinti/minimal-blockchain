use minimal_blockchain::blockchain::{
    chain::Blockchain, mempool::Mempool, transaction::Transaction,
};

fn tx(data: &[u8]) -> Transaction {
    Transaction::new(data.to_vec())
}

#[test]
fn transactions_are_added_to_mempool() {
    let mut mempool = Mempool::new();

    mempool.add_transaction(tx(b"a"));
    mempool.add_transaction(tx(b"b"));

    assert_eq!(mempool.len(), 2);
}

#[test]
fn assembling_block_drains_mempool() {
    let mut chain = Blockchain::init();
    let mut mempool = Mempool::new();

    mempool.add_transaction(tx(b"a"));
    mempool.add_transaction(tx(b"b"));

    let block = chain.assemble_block(&mut mempool).unwrap();

    assert_eq!(block.transactions.len(), 2);
    assert_eq!(mempool.len(), 0);
    assert_eq!(chain.tip().height, 1);
}

#[test]
fn assembling_empty_mempool_fails() {
    let mut chain = Blockchain::init();
    let mut mempool = Mempool::new();

    assert!(chain.assemble_block(&mut mempool).is_err());
}

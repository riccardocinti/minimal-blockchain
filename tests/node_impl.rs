use minimal_blockchain::blockchain::chain::Blockchain;
use minimal_blockchain::blockchain::mempool::Mempool;
use minimal_blockchain::blockchain::transaction::Transaction;
use minimal_blockchain::node::node::Node;
use minimal_blockchain::node::node_config::NodeConfig;

fn tx(data: &[u8]) -> Transaction {
    Transaction::new(data.to_vec())
}

#[test]
fn node_creation_successful() {
    let node = Node::new(Blockchain::init(), Mempool::new(), NodeConfig::new(false));

    assert_eq!(node.mempool.len(), 0);
    assert_eq!(node.chain().tip().block_hash, "GENESIS");
}

#[test]
fn transaction_is_submitted_to_mempool() {
    let mut node = Node::new(Blockchain::init(), Mempool::new(), NodeConfig::new(false));

    node.submit_transaction(tx(b"hello block"));

    assert_eq!(node.mempool.len(), 1);
    assert_eq!(node.chain().blocks.len(), 1);
}

#[test]
fn tick_does_nothing_for_empty_mempool() {
    let mut node = Node::new(Blockchain::init(), Mempool::new(), NodeConfig::new(false));

    node.tick().unwrap();

    assert_eq!(node.mempool.len(), 0);
    assert_eq!(node.chain().blocks.len(), 1);
}

#[test]
fn tick_creates_block_for_non_empty_mempool() {
    let mut node = Node::new(Blockchain::init(), Mempool::new(), NodeConfig::new(true));
    node.submit_transaction(tx(b"hello block"));

    node.tick().unwrap();

    assert_eq!(node.mempool.len(), 0);
    assert_eq!(node.chain().blocks.len(), 2);
    assert_eq!(node.chain().tip().height, 1);
}

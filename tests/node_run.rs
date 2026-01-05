use minimal_blockchain::blockchain::chain::Blockchain;
use minimal_blockchain::blockchain::mempool::Mempool;
use minimal_blockchain::blockchain::transaction::Transaction;
use minimal_blockchain::node::node::Node;
use minimal_blockchain::node::node_config::NodeConfig;

fn test_node() -> Node {
    let chain = Blockchain::init();
    let mempool = Mempool::new();
    let logica_time = 1;

    let config = NodeConfig {
        mining_enabled: true,
        difficulty: 1,
        block_time_delta: 5,
        tick_interval: 10,
    };

    Node::new(chain, mempool, config, logica_time)
}

#[test]
fn node_run_tick_does_not_panic() {
    let mut node = test_node();
    let _ = node.run_tick(1);
}

#[test]
fn node_run_tick_advances_chain_when_mining_is_enabled() {
    let mut node = test_node();

    node.mempool
        .add_transaction(Transaction::new(b"tx1".to_vec()));

    let _ = node.run_tick(1);

    assert_eq!(node.chain().tip().height, 1);
}


#[test]
fn node_run_tick_does_not_mine_empty_blocks() {
    let mut node = test_node();
    let _ = node.run_tick(1);
    assert_eq!(node.chain().tip().height, 0);
}

#[test]
fn multiple_ticks_create_multiple_blocks() {
    let mut node = test_node();

    node.mempool.add_transaction(Transaction::new(b"tx1".to_vec()));
    let _ = node.run_tick(1);

    node.mempool.add_transaction(Transaction::new(b"tx2".to_vec()));
    let _ = node.run_tick(1);

    assert_eq!(node.chain().tip().height, 2);
}
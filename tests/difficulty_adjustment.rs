use minimal_blockchain::blockchain::{
    chain::Blockchain, mempool::Mempool, transaction::Transaction,
};
use minimal_blockchain::node::{node::Node, node::NodeConfig};

const WINDOW: u64 = 2;
const INITIAL_DIFFICULTY: usize = 2;

fn setup_node(block_time_delta: u64) -> Node {
    let chain = Blockchain::init();
    let mempool = Mempool::new();

    let config = NodeConfig {
        mining_enabled: true,
        difficulty: INITIAL_DIFFICULTY,
        block_time_delta,
    };

    Node::new(chain, mempool, config, 1)
}

fn submit_dummy_tx(node: &mut Node, n: u64) {
    let payload = format!("tx-{}", n).into_bytes();
    let tx = Transaction::new(payload);
    node.submit_transaction(tx);
}

#[test]
fn difficulty_unchanged_before_window_is_full() {
    let mut node = setup_node(1); // normal speed

    for i in 0..WINDOW - 1 {
        submit_dummy_tx(&mut node, i);
        node.tick().unwrap();
    }

    let tip = node.chain().tip();
    assert_eq!(tip.difficulty, INITIAL_DIFFICULTY);
}

#[test]
fn difficulty_increases_when_blocks_are_too_fast() {
    let mut node = setup_node(2);

    for i in 0..WINDOW {
        submit_dummy_tx(&mut node, i);
        node.tick().unwrap();
    }

    let tip = node.chain().tip();
    assert!(tip.difficulty > INITIAL_DIFFICULTY);
}

#[test]
fn difficulty_decreases_when_blocks_are_too_slow() {
    let mut node = setup_node(1);

    for i in 0..WINDOW {
        submit_dummy_tx(&mut node, i);
        node.tick().unwrap();
        node.logical_time += 10;
    }

    let tip = node.chain().tip();
    assert!(tip.difficulty < INITIAL_DIFFICULTY);
}

#[test]
fn difficulty_never_drops_below_one() {
    let mut node = setup_node(1);

    for i in 0..WINDOW {
        submit_dummy_tx(&mut node, i);
        node.tick().unwrap();
        node.logical_time += 10;
    }

    let tip = node.chain().tip();
    assert!(tip.difficulty >= 1);
}

use minimal_blockchain::blockchain::{block::Block, chain::Blockchain, transaction::Transaction};

const TEST_DIFFICULTY: usize = 2;

#[test]
fn valid_pow_block_is_accepted() {
    let mut chain = Blockchain::init();
    let tx = Transaction::new(b"hello".to_vec());

    let block = Block::mine(chain.tip(), vec![tx], TEST_DIFFICULTY);

    let result = chain.receive_block(block);

    assert!(result.is_ok());
}

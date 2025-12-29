use crate::blockchain::block::Block;

pub struct State {}

impl State {
    pub fn replay(chain: &[Block]) {
        for block in chain {
            for tx in &block.transactions {
                let _ = tx;
            }
        }
    }
}

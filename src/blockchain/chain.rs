use crate::blockchain::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new(genesis: Block) -> Self {
        Self {
            blocks: vec![genesis],
        }
    }

    pub fn init() -> Self {
        Self {
            blocks: vec![Block::genesis()],
        }
    }

    pub fn tip(&self) -> &Block {
        self.blocks.last().expect("chain is never empty")
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_initialization_creates_single_block() {
        let chain = Blockchain::init();

        assert_eq!(
            chain.blocks.len(),
            1,
            "Blockchain must start with exactly one block."
        );
    }

    #[test]
    fn chain_is_never_empty() {
        let chain = Blockchain::init();
        assert!(chain.blocks.is_empty(), "Blockchain must not be empty");
    }
}

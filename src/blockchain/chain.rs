use crate::blockchain::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn init() -> Self {
        Self {
            blocks: vec![Block::genesis()],
        }
    }

    pub fn tip(&self) -> &Block {
        self.blocks.last().expect("chain is never empty")
    }

    pub fn append_block(&mut self, block: Block) -> Result<(), ChainError> {
        Self::check_height(self.tip(), &block)?;
        Self::check_previous_hash(self.tip(), &block)?;
        Self::check_block_hash(&block)?;
        Self::check_not_genesis(&block, &self.blocks[0])?;

        self.blocks.push(block);
        Ok(())
    }

    pub fn validate(&self) -> Result<(), ChainError> {
        if self.blocks.is_empty() {
            return Err(ChainError::EmptyChain);
        }
        if self.blocks[0] != Block::genesis() {
            return Err(ChainError::InvalidGenesis);
        }

        for pair in self.blocks.windows(2) {
            let prev = &pair[0];
            let curr = &pair[1];
            Self::check_height(prev, curr)?;
            Self::check_previous_hash(prev, curr)?;
            Self::check_block_hash(curr)?;
        }

        Ok(())
    }

    fn check_height(tip: &Block, block: &Block) -> Result<(), ChainError> {
        (block.height == tip.height + 1)
            .then_some(())
            .ok_or(ChainError::InvalidHeight)
    }

    fn check_previous_hash(tip: &Block, block: &Block) -> Result<(), ChainError> {
        (block.previous_hash == tip.block_hash)
            .then_some(())
            .ok_or(ChainError::InvalidPreviousHash)
    }

    fn check_block_hash(block: &Block) -> Result<(), ChainError> {
        Block::is_hash_valid(block)
            .then_some(())
            .ok_or(ChainError::InvalidBlockHash)
    }

    fn check_not_genesis(block: &Block, genesis: &Block) -> Result<(), ChainError> {
        (block.block_hash != genesis.block_hash)
            .then_some(())
            .ok_or(ChainError::GenesisAlreadyExists)
    }
}

pub enum ChainError {
    InvalidHeight,
    InvalidPreviousHash,
    InvalidBlockHash,
    GenesisAlreadyExists,
    EmptyChain,
    InvalidGenesis,
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
        assert!(!chain.blocks.is_empty(), "Blockchain must not be empty");
    }

    #[test]
    fn genesis_block_cannot_be_appended_again() {
        let mut chain = Blockchain::init();
        let genesis = chain.blocks[0].clone();

        assert!(
            chain.append_block(genesis).is_err(),
            "Genesis block must not be appended again"
        );
    }
}

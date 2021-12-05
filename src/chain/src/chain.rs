use crate::block::Block;

pub struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self {
        let mut block_chain = Blockchain { chain: Vec::new() };
        let genesis_block = Block::genesis();
        block_chain.chain.push(genesis_block);

        block_chain
    }

    fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_new_block(&mut self, new_block: Block) {
        self.chain.push(new_block)
    }

}
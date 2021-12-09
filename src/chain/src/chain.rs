use crate::block::Block;

#[derive(Clone)]
pub struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self {
        let mut new_chain = Blockchain { chain: Vec::new() };
        new_chain.genesis_block();
        new_chain
    }

    pub fn genesis_block(&mut self) {
        let genesis_block = Block::genesis();
        self.chain.push(genesis_block);
    }

    pub fn last_block(&self) -> &Block {
        match self.chain.last() {
            Some(block) => block,
            None => {
                panic!("No blocks in blockchain")
            }
        }
    }

    pub fn get_block(&self, index: i32) -> &Block {
        &self.chain[index as usize]
    }

    pub fn add_new_block(&mut self, new_block: Block) {
        self.chain.push(new_block)
    }

    pub fn sync_chain(&mut self, chain: Vec<Block>) {
        self.chain = chain
    }

}
use crate::block::Block;

#[derive(Clone, Debug, Default)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn is_valid(&self) -> bool {
        for (index, block) in self.blocks.iter().enumerate() {
            if !block.verify_block() {
                return false;
            }

            if index > 0 && block.prev_hash != self.blocks[index - 1].block_hash {
                return false;
            }
        }

        true
    }

    pub fn get_latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn get_block(&self, index: usize) -> Option<&Block> {
        self.blocks.get(index)
    }
}

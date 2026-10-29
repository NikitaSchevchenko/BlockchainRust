use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: usize,
    pub blockchain: Blockchain,
    pub mempool: Vec<Transaction>,
    pub peers: Vec<usize>,
}

impl Node {
    pub fn new(id: usize, genesis_block: Option<Block>) -> Self {
        let mut blockchain = Blockchain::new();

        if let Some(block) = genesis_block {
            blockchain.add_block(block);
        }

        Self {
            id,
            blockchain,
            mempool: Vec::new(),
            peers: Vec::new(),
        }
    }

    pub fn connect_peer(&mut self, peer_id: usize) {
        if !self.peers.contains(&peer_id) {
            self.peers.push(peer_id);
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.mempool.push(transaction);
    }

    pub fn mine_block(&mut self, difficulty: &str) -> Option<Block> {
        if self.blockchain.blocks.is_empty() {
            return None;
        }

        let previous_block = self.blockchain.get_latest_block().unwrap();
        let transactions = self.mempool.clone();

        let mut block = Block::new(
            1,
            previous_block.block_hash.clone(),
            transactions,
            difficulty.to_string(),
        );

        block.mine();
        self.blockchain.add_block(block.clone());
        self.mempool.clear();

        Some(block)
    }

    pub fn receive_block(&mut self, block: &Block) -> bool {
        if !block.verify_block() {
            return false;
        }

        if let Some(last) = self.blockchain.get_latest_block() {
            if block.prev_hash != last.block_hash {
                return false;
            }
        }

        self.blockchain.add_block(block.clone());
        true
    }

    pub fn broadcast_block(&self, block: &Block, peers: &mut [Node]) {
        for peer in peers.iter_mut() {
            if peer.id != self.id {
                peer.receive_block(block);
            }
        }
    }
}

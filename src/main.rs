mod crypto;
mod transaction;
mod block;
mod blockchain;

use block::Block;
use blockchain::Blockchain;
use transaction::Transaction;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transaction_hash_and_signature_are_valid() {
        let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 100);
        assert!(tx.verify_hash());
        assert!(tx.verify_signature());
    }

    #[test]
    fn block_hash_and_signature_are_valid() {
        let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 100);
        let tx2 = Transaction::new("Bob".to_string(), "Charlie".to_string(), 50);
        let block = Block::new(1, "0000000000000000".to_string(), vec![tx1, tx2], "0000".to_string());
        assert!(block.verify_block());
    }

    #[test]
    fn blockchain_validates_linked_blocks() {
        let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 100);
        let tx2 = Transaction::new("Bob".to_string(), "Charlie".to_string(), 50);
        let tx3 = Transaction::new("Charlie".to_string(), "David".to_string(), 25);
        let tx4 = Transaction::new("David".to_string(), "Alice".to_string(), 10);

        let block1 = Block::new(1, "0000000000000000".to_string(), vec![tx1, tx2], "0000".to_string());
        let block2 = Block::new(1, block1.block_hash.clone(), vec![tx3, tx4], "0000".to_string());

        let mut chain = Blockchain::new();
        chain.add_block(block1);
        chain.add_block(block2);

        assert!(chain.is_valid());
        assert_eq!(chain.get_latest_block().unwrap().block_hash, chain.blocks.last().unwrap().block_hash);
    }
}

fn main() {
    println!("===== TRANSACTIONS =====");

    let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 100);
    let tx2 = Transaction::new("Bob".to_string(), "Charlie".to_string(), 50);
    let tx3 = Transaction::new("Charlie".to_string(), "David".to_string(), 25);
    let tx4 = Transaction::new("David".to_string(), "Alice".to_string(), 10);

    println!("{}", tx1.to_string());
    println!("Transaction 1 hash:");
    println!("{}", tx1.verify_hash());
    println!("Transaction 1 signature:");
    println!("{}", tx1.verify_signature());

    println!("\n===== BLOCKS =====");

    let block1 = Block::new(1, "0000000000000000".to_string(), vec![tx1.clone(), tx2.clone()], "0000".to_string());
    let block2 = Block::new(1, block1.block_hash.clone(), vec![tx3.clone(), tx4.clone()], "0000".to_string());

    println!("{}", block1.to_string());
    println!("Block 1 verification:");
    println!("{}", block1.verify_block());

    println!("{}", block2.to_string());
    println!("Block 2 verification:");
    println!("{}", block2.verify_block());

    let mut blockchain = Blockchain::new();
    blockchain.add_block(block1);
    blockchain.add_block(block2);

    println!("\n===== BLOCKCHAIN =====");
    for block in &blockchain.blocks {
        println!("{}", block.to_string());
    }

    println!("Latest block:");
    println!("{}", blockchain.get_latest_block().unwrap().to_string());
    println!("Blockchain valid:");
    println!("{}", blockchain.is_valid());
}

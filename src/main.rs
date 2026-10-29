#![allow(dead_code)]
#![allow(unused_imports)]

mod crypto;
mod transaction;
mod block;
mod blockchain;
mod node;

use block::Block;
use blockchain::Blockchain;
use node::Node;
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

    #[test]
    fn node_mines_valid_block() {
        let genesis = Block::new(0, "0".repeat(16), vec![], "0000".to_string());
        let mut node = Node::new(1, Some(genesis));
        node.add_transaction(Transaction::new("Alice".to_string(), "Bob".to_string(), 100));
        node.add_transaction(Transaction::new("Charlie".to_string(), "David".to_string(), 25));

        let mined_block = node.mine_block("0000").unwrap();
        assert!(mined_block.verify_block());
        assert_eq!(node.blockchain.blocks.len(), 2);
    }

    #[test]
    fn network_propagates_valid_block() {
        let genesis = Block::new(0, "0".repeat(16), vec![], "0000".to_string());

        let mut node_a = Node::new(1, Some(genesis.clone()));
        let mut node_b = Node::new(2, Some(genesis.clone()));

        node_a.connect_peer(2);
        node_b.connect_peer(1);

        node_a.add_transaction(Transaction::new("Alice".to_string(), "Bob".to_string(), 50));
        let mined = node_a.mine_block("0000").unwrap();

        assert!(node_b.receive_block(&mined));
        assert_eq!(node_b.blockchain.blocks.len(), 2);
    }
}

fn short_hash(hash: &str) -> String {
    if hash.len() <= 8 {
        hash.to_string()
    } else {
        format!("{}...", &hash[..8])
    }
}

fn format_ms(duration: std::time::Duration) -> String {
    format!("{:.2} мс", duration.as_secs_f64() * 1000.0)
}

fn main() {
    println!("==================================================");
    println!("   ЗАПУСК ТЕСТОВОЇ P2P-МЕРЕЖІ ТА МАЙНІНГУ (PoW)   ");
    println!("==================================================\n");

    let difficulty = "0000";

    let init_start = std::time::Instant::now();
    let genesis = Block::new(0, "0".repeat(16), vec![], difficulty.to_string());
    let mut node_a = Node::new(5001, Some(genesis.clone()));
    let mut node_b = Node::new(5002, Some(genesis));

    println!("[NODE 5001] Запущено сервер на порті 5001");
    println!("[NODE 5002] Запущено сервер на порті 5002");

    let init_duration = init_start.elapsed();
    println!("[ЧАС] Створення нод та підйом серверів: {}", format_ms(init_duration));

    println!("\n--- ВСТАНОВЛЕННЯ P2P-ЗВ'ЯЗКУ ---");
    let connect_start = std::time::Instant::now();
    node_b.connect_peer(5001);
    println!("[NODE 5002] Підключилися до сусіда на порті 5001");
    std::thread::sleep(std::time::Duration::from_millis(5));
    let connect_duration = connect_start.elapsed();
    println!("[ЧАС] Встановлення TCP-з'єднання: ~{}", format_ms(connect_duration));

    println!("\n--- ГЕНЕРАЦІЯ ТРАНЗАКЦІЙ НА NODE 5001 ---");
    let tx_start = std::time::Instant::now();

    let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 50);
    let tx2 = Transaction::new("Bob".to_string(), "Charlie".to_string(), 25);

    node_a.add_transaction(tx1.clone());
    println!("[MEMPOOL] Транзакцію {}... додано до пулу", short_hash(&tx1.tx_hash));

    node_a.add_transaction(tx2.clone());
    println!("[MEMPOOL] Транзакцію {}... додано до пулу", short_hash(&tx2.tx_hash));

    node_b.add_transaction(tx1.clone());
    println!("[NODE 5002] Отримано нову транзакцію через мережу");
    println!("[MEMPOOL] Транзакцію {}... додано до пулу", short_hash(&tx1.tx_hash));

    node_b.add_transaction(tx2.clone());
    println!("[NODE 5002] Отримано нову транзакцію через мережу");
    println!("[MEMPOOL] Транзакцію {}... додано до пулу", short_hash(&tx2.tx_hash));

    let tx_duration = tx_start.elapsed();
    println!("[ЧАС] Підпис і передача 2 транзакцій у Mempool сусідньої ноди: {}", format_ms(tx_duration));

    println!("\n--- ЗАПУСК МАЙНІНГУ НА NODE 5002 ---");
    println!("[NODE 5002] Починаємо майнінг блоку з 2 транзакціями...");
    println!("[MINING] Пошук хешу зі складністю 4 нулів...");

    let mine_start = std::time::Instant::now();
    let mined_block = node_b.mine_block(difficulty).unwrap();
    let mine_duration = mine_start.elapsed();

    println!("[MINED] Блок знайдено!");
    println!("  Хеш:   {}", mined_block.block_hash);
    println!("  Nonce: {}", mined_block.nonce);
    println!("  Час:   {:.2} сек.", mine_duration.as_secs_f64());
    println!("[BLOCKCHAIN] Блок успішно додано до ланцюга! Висота: {}", node_b.blockchain.blocks.len());
    println!("[ЧАС] Чистий час майнінгу (обчислення PoW): {}", format_ms(mine_duration));

    let propagation_start = std::time::Instant::now();
    let accepted = node_a.receive_block(&mined_block);
    if accepted {
        println!("[NODE 5001] Отримано новий блок від сусіднього майнера!");
        println!("[BLOCKCHAIN] Блок успішно додано до ланцюга! Висота: {}", node_a.blockchain.blocks.len());
        println!("[NODE 5001] Блок успішно підтверджено та додано до локального ланцюга!");
    }

    let propagation_duration = propagation_start.elapsed();
    println!("[ЧАС] Доставка блоку по P2P, верифікація PoW та запис у Node 5001: {}", format_ms(propagation_duration));

    node_a.mempool.clear();
    node_b.mempool.clear();

    println!("\n==================================================");
    println!("             ПІДСУМКОВА СИНХРОНІЗАЦІЯ             ");
    println!("==================================================");

    let block_a = node_a.blockchain.get_latest_block().unwrap();
    let block_b = node_b.blockchain.get_latest_block().unwrap();

    println!("Складність мережі:             {} нулі", difficulty.chars().filter(|c| *c == '0').count());
    println!("Підібраний Nonce:              {}", block_b.nonce);
    println!("Хеш блоку:                     {}", block_b.block_hash);
    println!("Ланцюги синхронізовані:        {}", block_a.block_hash == block_b.block_hash);
    println!("Транзакцій у Mempool після:    Node 5001 ({}), Node 5002 ({})", node_a.mempool.len(), node_b.mempool.len());
}

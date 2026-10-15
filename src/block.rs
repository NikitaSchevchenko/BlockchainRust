use rand::rngs::OsRng;
use secp256k1::{
    ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey,
};

use crate::crypto::{sha256_digest, sha256_hex};
use crate::transaction::Transaction;

#[derive(Clone, Debug)]
pub struct Block {
    pub version: u64,
    pub prev_hash: String,
    pub timestamp: u128,
    pub difficulty_target: String,
    pub nonce: u64,
    pub merkle_root: String,
    pub transactions: Vec<Transaction>,
    pub block_hash: String,
    pub signature: Vec<u8>,
    private_key: SecretKey,
    public_key: PublicKey,
}

impl Block {
    pub fn new(
        version: u64,
        prev_hash: String,
        transactions: Vec<Transaction>,
        difficulty_target: String,
    ) -> Self {
        let secp = Secp256k1::new();
        let private_key = SecretKey::new(&mut OsRng);
        let public_key = PublicKey::from_secret_key(&secp, &private_key);

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let mut block = Self {
            version,
            prev_hash,
            timestamp,
            difficulty_target,
            nonce: 0,
            merkle_root: String::new(),
            transactions,
            block_hash: String::new(),
            signature: Vec::new(),
            private_key,
            public_key,
        };

        block.merkle_root = block.calculate_merkle_root();
        block.block_hash = block.calculate_hash();
        block.sign_block();
        block
    }

    fn calculate_hash_pair(hash1: &str, hash2: &str) -> String {
        let data = format!("{}{}", hash1, hash2);
        sha256_hex(data.as_bytes())
    }

    pub fn calculate_merkle_root(&self) -> String {
        let mut hashes: Vec<String> = self
            .transactions
            .iter()
            .map(|tx| tx.tx_hash.clone())
            .collect();

        if hashes.is_empty() {
            return sha256_hex(b"");
        }

        while hashes.len() > 1 {
            let mut new_level = Vec::new();

            for i in (0..hashes.len()).step_by(2) {
                let left = hashes[i].clone();
                let right = hashes.get(i + 1).cloned().unwrap_or_else(|| left.clone());
                new_level.push(Self::calculate_hash_pair(&left, &right));
            }

            hashes = new_level;
        }

        hashes[0].clone()
    }

    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{{\"version\":{},\"prevHash\":\"{}\",\"timestamp\":{},\"difficultyTarget\":\"{}\",\"nonce\":{},\"merkleRoot\":\"{}\"}}",
            self.version,
            self.prev_hash,
            self.timestamp,
            self.difficulty_target,
            self.nonce,
            self.merkle_root
        );

        sha256_hex(data.as_bytes())
    }

    pub fn sign_block(&mut self) {
        let secp = Secp256k1::new();
        let message_digest = sha256_digest(self.block_hash.as_bytes());
        let message = Message::from_digest_slice(&message_digest)
            .expect("The block hash is a valid message digest.");
        let signature = secp.sign_ecdsa(&message, &self.private_key);
        self.signature = signature.serialize_der().to_vec();
    }

    pub fn verify_signature(&self) -> bool {
        let secp = Secp256k1::new();
        let message_digest = sha256_digest(self.block_hash.as_bytes());
        let message = Message::from_digest_slice(&message_digest)
            .expect("The block hash is a valid message digest.");
        let signature = Signature::from_der(&self.signature)
            .expect("Block signature should be a valid DER-encoded ECDSA signature.");

        secp.verify_ecdsa(&message, &signature, &self.public_key).is_ok()
    }

    pub fn verify_block(&self) -> bool {
        let current_hash = self.calculate_hash();
        let current_merkle_root = self.calculate_merkle_root();

        current_hash == self.block_hash
            && current_merkle_root == self.merkle_root
            && self.verify_signature()
    }

    pub fn set_version(&mut self, value: u64) {
        self.version = value;
        self.block_hash = self.calculate_hash();
        self.sign_block();
    }

    pub fn set_prev_hash(&mut self, value: String) {
        self.prev_hash = value;
        self.block_hash = self.calculate_hash();
        self.sign_block();
    }

    pub fn set_timestamp(&mut self, value: u128) {
        self.timestamp = value;
        self.block_hash = self.calculate_hash();
        self.sign_block();
    }

    pub fn set_difficulty_target(&mut self, value: String) {
        self.difficulty_target = value;
        self.block_hash = self.calculate_hash();
        self.sign_block();
    }

    pub fn set_nonce(&mut self, value: u64) {
        self.nonce = value;
        self.block_hash = self.calculate_hash();
        self.sign_block();
    }

    pub fn set_merkle_root(&mut self, value: String) {
        self.merkle_root = value;
        self.block_hash = self.calculate_hash();
        self.sign_block();
    }

    pub fn set_transactions(&mut self, value: Vec<Transaction>) {
        self.transactions = value;
        self.merkle_root = self.calculate_merkle_root();
        self.block_hash = self.calculate_hash();
        self.sign_block();
    }

    pub fn to_string(&self) -> String {
        let txs = self
            .transactions
            .iter()
            .map(|tx| tx.to_string())
            .collect::<Vec<_>>()
            .join(",");

        format!(
            "{{\"version\":{},\"prevHash\":\"{}\",\"timestamp\":{},\"difficultyTarget\":\"{}\",\"nonce\":{},\"merkleRoot\":\"{}\",\"transactions\":[{}],\"blockHash\":\"{}\"}}",
            self.version,
            self.prev_hash,
            self.timestamp,
            self.difficulty_target,
            self.nonce,
            self.merkle_root,
            txs,
            self.block_hash
        )
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
}

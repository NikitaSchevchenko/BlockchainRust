use rand::rngs::OsRng;
use secp256k1::{
    ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey,
};

use crate::crypto::{sha256_digest, sha256_hex};

#[derive(Clone, Debug)]
pub struct Transaction {
    pub input: String,
    pub output: String,
    pub amount: u64,
    pub tx_timestamp: u128,
    pub tx_hash: String,
    pub signature: Vec<u8>,
    private_key: SecretKey,
    public_key: PublicKey,
}

impl Transaction {
    pub fn new(input: String, output: String, amount: u64) -> Self {
        let secp = Secp256k1::new();
        let private_key = SecretKey::new(&mut OsRng);
        let public_key = PublicKey::from_secret_key(&secp, &private_key);

        let tx_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let mut tx = Self {
            input,
            output,
            amount,
            tx_timestamp,
            tx_hash: String::new(),
            signature: Vec::new(),
            private_key,
            public_key,
        };

        tx.tx_hash = tx.calculate_hash();
        tx.sign_transaction();
        tx
    }

    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{{\"input\":\"{}\",\"output\":\"{}\",\"amount\":{},\"txTimestamp\":{}}}",
            self.input, self.output, self.amount, self.tx_timestamp
        );
        sha256_hex(data.as_bytes())
    }

    pub fn verify_hash(&self) -> bool {
        self.calculate_hash() == self.tx_hash
    }

    pub fn sign_transaction(&mut self) {
        let secp = Secp256k1::new();
        let message_digest = sha256_digest(self.tx_hash.as_bytes());
        let message = Message::from_digest_slice(&message_digest)
            .expect("The transaction hash is a valid message digest.");
        let signature = secp.sign_ecdsa(&message, &self.private_key);
        self.signature = signature.serialize_der().to_vec();
    }

    pub fn verify_signature(&self) -> bool {
        let secp = Secp256k1::new();
        let message_digest = sha256_digest(self.tx_hash.as_bytes());
        let message = Message::from_digest_slice(&message_digest)
            .expect("The transaction hash is a valid message digest.");
        let signature = Signature::from_der(&self.signature)
            .expect("Transaction signature should be a valid DER-encoded ECDSA signature.");

        secp.verify_ecdsa(&message, &signature, &self.public_key).is_ok()
    }

    pub fn set_input(&mut self, value: String) {
        self.input = value;
        self.tx_hash = self.calculate_hash();
        self.sign_transaction();
    }

    pub fn set_output(&mut self, value: String) {
        self.output = value;
        self.tx_hash = self.calculate_hash();
        self.sign_transaction();
    }

    pub fn set_amount(&mut self, value: u64) {
        self.amount = value;
        self.tx_hash = self.calculate_hash();
        self.sign_transaction();
    }

    pub fn set_tx_timestamp(&mut self, value: u128) {
        self.tx_timestamp = value;
        self.tx_hash = self.calculate_hash();
        self.sign_transaction();
    }

    pub fn set_tx_hash(&mut self, value: String) {
        self.tx_hash = value;
        self.sign_transaction();
    }

    pub fn to_string(&self) -> String {
        format!(
            "{{\"input\":\"{}\",\"output\":\"{}\",\"amount\":{},\"txTimestamp\":{},\"txHash\":\"{}\"}}",
            self.input, self.output, self.amount, self.tx_timestamp, self.tx_hash
        )
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
}

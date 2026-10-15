use sha2::{Digest, Sha256};

pub fn sha256_hex(input: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    format!("{:x}", hasher.finalize())
}

pub fn sha256_digest(input: &[u8]) -> [u8; 32] {
    let digest = Sha256::digest(input);
    let mut result = [0u8; 32];
    result.copy_from_slice(digest.as_slice());
    result
}

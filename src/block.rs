use crate::tx::Tx;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub hash: String,
    pub prev_hash: String,
    pub data: Vec<Tx>,
    pub merkle_root: String,
    pub difficulty: u32,
    pub nonce: u32,
}

impl Block {
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = format!(
            "{}{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.prev_hash,
            self.merkle_root,
            self.difficulty,
            self.nonce
        );
        hasher.input_str(&data);
        hasher.result_str()
    }

    pub fn calculate_merkle_root(&self) -> String {
        let mut hasher = Sha256::new();
        let mut data = "".to_string();
        for tx in &self.data {
            data.push_str(&tx.hash);
        }
        hasher.input_str(&data);
        hasher.result_str()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.hash != self.calculate_hash() {
            return Err("Invalid hash".to_string());
        }
        if self.merkle_root != self.calculate_merkle_root() {
            return Err("Invalid merkle root".to_string());
        }
        Ok(())
    }
}

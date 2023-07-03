use crate::tx::Tx;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::{Deserialize, Serialize};

/// Represents a block in the blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// The index of the block.
    pub index: u32,
    /// The timestamp of the block.
    pub timestamp: u64,
    /// The hash of the block.
    pub hash: String,
    /// The hash of the previous block.
    pub prev_hash: String,
    /// The transactions included in the block.
    pub data: Vec<Tx>,
    /// The Merkle root of the transactions in the block.
    pub merkle_root: String,
    /// The difficulty of mining the block.
    pub difficulty: u32,
    /// The nonce used in mining the block.
    pub nonce: u32,
}

impl Block {
    /// Calculates the hash of the block based on its properties.
    ///
    /// # Returns
    ///
    /// The calculated hash as a hexadecimal string.
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

    /// Calculates the Merkle root of the transactions in the block.
    ///
    /// # Returns
    ///
    /// The calculated Merkle root as a hexadecimal string.
    pub fn calculate_merkle_root(&self) -> String {
        let mut hasher = Sha256::new();
        let mut data = String::new();
        for tx in &self.data {
            data.push_str(&tx.hash);
        }
        hasher.input_str(&data);
        hasher.result_str()
    }

    /// Validates the block.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the block is valid.
    /// - `Err(String)` with an error message if the block is invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.hash != self.calculate_hash() {
            return Err("Invalid hash".to_string());
        }
        if self.merkle_root != self.calculate_merkle_root() {
            return Err("Invalid Merkle root".to_string());
        }
        Ok(())
    }
}


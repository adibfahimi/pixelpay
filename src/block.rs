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
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.prev_hash,
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
            return Err("Invalid hash".to_owned());
        }

        if self.hash.len() != 64 {
            return Err("Invalid hash length".to_owned());
        }

        if self.prev_hash.len() != 64 {
            return Err("Invalid previous hash length".to_owned());
        }

        if self.data.is_empty() {
            return Err("Data is empty".to_owned());
        }

        if self.merkle_root != self.calculate_merkle_root() {
            return Err("Invalid Merkle root".to_owned());
        }

        for tx in &self.data {
            tx.validate()?;
        }

        let current_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if self.timestamp > current_timestamp {
            return Err("Invalid timestamp".to_owned());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tx::Tx;

    #[test]
    fn test_validate_block() {
        let tx = Tx {
            sender: "03fe5eae6c815eb76624e0c289c3f579339710eaaf7edd84a39eee23cf54bdc07a".to_string(),
            receiver: "02e7224daa0ddc3aab07bba9fbab024af330b1fc44c07356efba7e24e4e4ae4045".to_string(),
            amount: 10,
            signature: "3045022100ce73d8ef925eae99766a41ff00ae8d1164e34013db705bebaacb5b36334adcd70220709a4229c36f3fb25bc58a991d85f82f764192afa8600487c8eb1125967ed24a".to_string(),
            hash: "fd23f4a373f5d22068a07434904fa1e1b8be4c14c6d87c5b94761c452517eb6b".to_string(),
            timestamp: 1625745632,
        };

        let mut block = Block {
            index: 0,
            timestamp: 1625745632,
            prev_hash: "92d01e28f93d1ff057f8efc4e7613ae0e32835f683f2f19cdd66f2ac5d8dce20".to_string(),
            data: vec![tx],
            hash: "".to_string(),
            merkle_root: "".to_string(),
            difficulty: 4,
            nonce: 0,
        };

        block.hash = block.calculate_hash();
        block.merkle_root = block.calculate_merkle_root();

        let result = block.validate();
        assert!(result.is_ok());
    }
}

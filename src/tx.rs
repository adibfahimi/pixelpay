use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::{Deserialize, Serialize};

/// Represents a transaction in the blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tx {
    /// The sender of the transaction.
    pub sender: String,
    /// The receiver of the transaction.
    pub receiver: String,
    /// The amount of the transaction.
    pub amount: u32,
    /// The signature of the transaction.
    pub signature: String,
    /// The hash of the transaction.
    pub hash: String,
    /// The timestamp of the transaction.
    pub timestamp: u64,
}

impl Tx {
    /// Calculates the hash of the transaction based on its properties.
    ///
    /// # Returns
    ///
    /// The calculated hash as a hexadecimal string.
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = format!(
            "{}{}{}{}",
            self.sender, self.receiver, self.amount, self.timestamp
        );
        hasher.input_str(&data);
        hasher.result_str()
    }

    /// Validates the transaction.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the transaction is valid.
    /// - `Err(String)` with an error message if the transaction is invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.hash != self.calculate_hash() {
            return Err("Invalid hash".to_string());
        }

        if self.sender == self.receiver {
            return Err("Sender and receiver cannot be the same".to_string());
        }

        if self.signature == *"" {
            return Err("Signature cannot be empty".to_string());
        }

        Ok(())
    }
}


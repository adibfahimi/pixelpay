use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tx {
    pub sender: String,
    pub receiver: String,
    pub amount: u32,
    pub signature: String,
    pub hash: String,
    pub timestamp: u64,
}

impl Tx {
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = format!(
            "{}{}{}{}",
            self.sender, self.receiver, self.amount, self.timestamp
        );
        hasher.input_str(&data);
        hasher.result_str()
    }

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

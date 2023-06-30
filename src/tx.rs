use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tx {
    pub sender: String,
    pub receiver: String,
    pub amount: i32,
    pub signature: String,
    pub hash: String,
    pub timestamp: u64,
}

impl Tx {
    // pub fn new(
    //     sender: String,
    //     receiver: String,
    //     amount: f32,
    //     signature: String,
    //     hash: String,
    //     timestamp: u64,
    // ) -> Tx {
    //     Tx {
    //         sender,
    //         receiver,
    //         amount,
    //         signature,
    //         hash,
    //         timestamp,
    //     }
    // }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = format!(
            "{}{}{}{}{}",
            self.sender, self.receiver, self.amount, self.signature, self.timestamp
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

        if self.amount <= 0 {
            return Err("Amount must be greater than 0".to_string());
        }

        if self.signature == "".to_string() {
            return Err("Signature cannot be empty".to_string());
        }

        if self.timestamp <= 0 {
            return Err("Timestamp must be greater than 0".to_string());
        }

        Ok(())
    }
}

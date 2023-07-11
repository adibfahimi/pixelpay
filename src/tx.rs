use crypto::digest::Digest;
use crypto::sha2::Sha256;
use secp256k1::{ecdsa, Message, PublicKey, Secp256k1};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
        let data = format!("{}{}{}{}", self.sender, self.receiver, self.amount, self.timestamp);
        hasher.input_str(&data);
        hasher.result_str()
    }

    /// Validates the transaction.
    ///
    /// This function checks the integrity and validity of a transaction by performing various checks on its properties.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the transaction is valid.
    /// - `Err(String)` with an error message if the transaction is invalid.
    ///
    /// # Errors
    ///
    /// The function can return an error with the following messages:
    ///
    /// - "Invalid hash": If the calculated hash of the transaction does not match the stored hash.
    /// - "Invalid hash length": If the length of the hash field of the transaction is not 64 characters.
    /// - "Sender cannot be empty": If the sender field of the transaction is an empty string.
    /// - "Receiver cannot be empty": If the receiver field of the transaction is an empty string.
    /// - "Sender and receiver cannot be the same": If the sender and receiver fields of the transaction are the same.
    /// - "Amount must be greater than zero": If the amount field of the transaction is zero.
    /// - "Signature cannot be empty": If the signature field of the transaction is an empty string.
    /// - "Invalid public key": If there is an error decoding the sender's public key from a hexadecimal string.
    /// - "Invalid hash": If there is an error decoding the hash from a hexadecimal string.
    /// - "Invalid signature": If the provided signature is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tx::Tx;
    ///
    /// let transaction = Tx {
    ///     sender: "Alice".to_string(),
    ///     receiver: "Bob".to_string(),
    ///     amount: 10,
    ///     signature: "abc123".to_string(),
    ///     hash: "123abc".to_string(),
    ///     timestamp: 1625745632,
    /// };
    ///
    /// assert!(transaction.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.hash != self.calculate_hash() {
            return Err("Invalid hash".to_string());
        }

        if self.hash.len() != 64 {
            return Err("Invalid hash length".to_string());
        }

        if self.sender.is_empty() {
            return Err("Sender cannot be empty".to_string());
        }

        if self.receiver.is_empty() {
            return Err("Receiver cannot be empty".to_string());
        }

        if self.sender == self.receiver {
            return Err("Sender and receiver cannot be the same".to_string());
        }

        if self.amount == 0 {
            return Err("Amount must be greater than zero".to_string());
        }

        if self.signature.is_empty() {
            return Err("Signature cannot be empty".to_string());
        }

        let secp = Secp256k1::verification_only();

        let public_key_hex = match hex::decode(&self.sender) {
            Ok(pk) => pk,
            Err(_) => return Err("Invalid public key".to_string()),
        };

        let public_key = match PublicKey::from_slice(&public_key_hex) {
            Ok(pk) => pk,
            Err(_) => return Err("Invalid public key".to_string()),
        };

        let message_hex = match hex::decode(&self.hash) {
            Ok(msg) => msg,
            Err(_) => return Err("Invalid hash".to_string()),
        };

        let message = match Message::from_slice(&message_hex) {
            Ok(msg) => msg,
            Err(_) => return Err("Invalid hash".to_string()),
        };

        let signature = match ecdsa::Signature::from_str(&self.signature) {
            Ok(sig) => sig,
            Err(_) => return Err("Invalid signature".to_string()),
        };

        if secp.verify_ecdsa(&message, &signature, &public_key).is_err() {
            return Err("Invalid signature".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::rand::rngs::OsRng;

    #[test]
    fn test_validate_valid_tx() {
        let secp = Secp256k1::new();

        let (sk_a, pk_a) = secp.generate_keypair(&mut OsRng);
        let (_, pk_b) = secp.generate_keypair(&mut OsRng);

        let mut tx = Tx {
            sender: pk_a.to_string(),
            receiver: pk_b.to_string(),
            amount: 10,
            signature: "".to_string(),
            hash: "".to_string(),
            timestamp: 1625745632,
        };

        tx.hash = tx.calculate_hash();

        let message = Message::from_slice(&hex::decode(&tx.hash).unwrap()).unwrap();

        tx.signature = secp.sign_ecdsa(&message, &sk_a).to_string();
        tx.sender = hex::encode(pk_a.serialize());

        assert!(tx.validate().is_ok());
    }
}

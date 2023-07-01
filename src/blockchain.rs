use crate::block::Block;
use crate::tx::Tx;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub pending_txs: Vec<Tx>,
    pub difficulty: u32,
    pub mining_reward: u32,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut tx = Tx {
            sender: "".to_string(),
            receiver: "03247e21fbb8be9ee2b666d431f43c88c2dd8e422ee805dc6c08593de93b721d7b"
                .to_string(),
            amount: 1000,
            signature: "".to_string(),
            hash: "".to_string(),
            timestamp: 1688204859,
        };

        tx.hash = tx.calculate_hash();

        let mut genesis_block = Block {
            index: 0,
            timestamp: 1688062447,
            prev_hash: "".to_string(),
            merkle_root: "".to_string(),
            hash: "".to_string(),
            data: vec![tx],
            nonce: 0,
            difficulty: 4,
        };

        genesis_block.hash = genesis_block.calculate_hash();

        Blockchain {
            blocks: vec![genesis_block],
            pending_txs: vec![],
            difficulty: 4,
            mining_reward: 100,
        }
    }
}

use crate::block::Block;
use crate::tx::Tx;
use serde::{Deserialize, Serialize};

const BLOCK_RATE: u64 = 12 * 60; // 12 minutes

/// Represents a blockchain.
#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    /// The blocks in the blockchain.
    pub blocks: Vec<Block>,
    /// The pending transactions in the blockchain.
    pub pending_txs: Vec<Tx>,
    /// The difficulty level for mining blocks in the blockchain.
    pub difficulty: u32,
    /// The reward amount for mining a block in the blockchain.
    pub mining_reward: u32,
}

impl Blockchain {
    /// Creates a new instance of the blockchain with a genesis block.
    ///
    /// # Returns
    ///
    /// A new `Blockchain` instance.
    pub fn new() -> Self {
        let mut tx = Tx {
            sender: "".to_string(),
            receiver: "0378880d9fa9663f0a7261f819e86bd222dcd6796df2dd6451b9bb86c31dbaade7"
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

    pub fn calculate_difficulty(&self) -> u32 {
        let mut difficulty = self.difficulty;
        let last_block = self.blocks.last().unwrap();

        if last_block.index % 10 == 0 {
            if last_block.timestamp > last_block.timestamp + BLOCK_RATE {
                difficulty -= 1;
            } else {
                difficulty += 1;
            }
        }

        difficulty
    }
}

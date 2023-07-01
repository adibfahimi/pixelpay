use crate::block::Block;
use crate::tx::Tx;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub pending_txs: Vec<Tx>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut tx = Tx::new(
            "".to_string(),
            "03247e21fbb8be9ee2b666d431f43c88c2dd8e422ee805dc6c08593de93b721d7b".to_string(),
            1000,
            "".to_string(),
            "".to_string(),
            1688204859,
        );

        tx.hash = tx.calculate_hash();
        let mut genesis_block = Block::new(
            0,
            1688062447,
            "".to_string(),
            "".to_string(),
            vec![tx],
            "".to_string(),
            4,
            0,
        );

        genesis_block.hash = genesis_block.calculate_hash();

        return Blockchain {
            blocks: vec![genesis_block],
            pending_txs: vec![],
        };
    }
}

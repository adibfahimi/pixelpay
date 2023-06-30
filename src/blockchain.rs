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
        let mut genesis_block = Block::new(
            0,
            1688062447,
            "".to_string(),
            "".to_string(),
            vec![],
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

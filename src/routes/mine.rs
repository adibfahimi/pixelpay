use crate::block::Block;
use crate::blockchain::Blockchain;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time;

/// Response structure for `get_block` function.
#[derive(Serialize, Deserialize)]
struct NodeResp {
    pub block: Block,
    pub miner_reward: u32,
}

/// Retrieves a new block for mining from the blockchain.
///
/// The function retrieves the pending transactions from the blockchain and creates a new block
/// with the necessary information for mining. The block includes the index, timestamp,
/// previous hash, merkle root, nonce, difficulty, and the pending transactions. The function
/// returns the new block along with the mining reward amount as a JSON response.
///
/// # Arguments
///
/// * `bc` - A shared data reference to the blockchain wrapped in a Mutex.
///
/// # Returns
///
/// An HTTP response containing the new block and mining reward as JSON.
///
/// If there are no pending transactions in the blockchain, an HTTP response with the message
/// "No transactions to mine" is returned.
pub async fn get_block(bc: web::Data<Mutex<Blockchain>>) -> impl Responder {
    let bc_ref = bc.lock().unwrap();

    if bc_ref.pending_txs.is_empty() {
        return HttpResponse::Ok().body("No transactions to mine");
    }

    let timestamp = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let block = Block {
        index: bc_ref.blocks.len() as u32,
        timestamp,
        prev_hash: bc_ref.blocks.last().unwrap().hash.clone(),
        merkle_root: "".to_string(),
        nonce: 0,
        difficulty: 4,
        data: bc_ref.pending_txs.clone(),
        hash: "".to_string(),
    };

    let resp = NodeResp {
        block,
        miner_reward: bc_ref.mining_reward,
    };

    HttpResponse::Ok().json(&resp)
}

/// Adds a validated block to the blockchain.
///
/// The function takes a validated block and adds it to the blockchain. It also clears the
/// pending transactions in the blockchain. If the provided block fails validation, an HTTP
/// response with the validation error message is returned.
///
/// # Arguments
///
/// * `bc` - A shared data reference to the blockchain wrapped in a Mutex.
/// * `block` - The validated block to be added, provided as JSON.
///
/// # Returns
///
/// An HTTP response indicating the success or failure of adding the block.
///
/// If there are no pending transactions in the blockchain, an HTTP response with the message
/// "No transactions to mine" is returned.
pub async fn add_block(
    bc: web::Data<Mutex<Blockchain>>,
    block: web::Json<Block>,
) -> impl Responder {
    let mut bc_ref = bc.lock().unwrap();

    if bc_ref.pending_txs.is_empty() {
        return HttpResponse::Ok().body("No transactions to mine");
    }

    match block.validate() {
        Ok(_) => {
            bc_ref.blocks.push(block.into_inner());
            bc_ref.pending_txs = vec![];
            HttpResponse::Ok().body("Block added")
        }
        Err(e) => HttpResponse::Ok().body(e),
    }
}


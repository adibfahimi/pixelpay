use crate::block::Block;
use crate::blockchain::Blockchain;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time;

#[derive(Serialize, Deserialize)]
struct NodeResp {
    pub block: Block,
    pub miner_reward: u32,
}

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

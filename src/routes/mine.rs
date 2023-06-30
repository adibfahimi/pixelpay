use crate::block::Block;
use crate::blockchain::Blockchain;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;
use std::time;

pub async fn get_block(bc: web::Data<Mutex<Blockchain>>) -> impl Responder {
    let bc_ref = bc.lock().unwrap();

    if bc_ref.pending_txs.len() == 0 {
        return HttpResponse::Ok().body("No transactions to mine");
    }

    let timestamp = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let block = Block::new(
        bc_ref.blocks.len() as u32,
        timestamp,
        "".to_string(),
        bc_ref.blocks.last().unwrap().hash.clone(),
        bc_ref.pending_txs.clone(),
        "".to_string(),
        4,
        0,
    );
    HttpResponse::Ok().json(&block)
}

pub async fn add_block(
    bc: web::Data<Mutex<Blockchain>>,
    block: web::Json<Block>,
) -> impl Responder {
    let mut bc_ref = bc.lock().unwrap();

    if bc_ref.pending_txs.len() == 0 {
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

use crate::blockchain::Blockchain;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;

/// Retrieves a block by its hash from the blockchain.
///
/// # Arguments
///
/// * `bc` - The shared mutable state of the blockchain wrapped in a Mutex.
/// * `hash` - The hash of the block to retrieve.
///
/// # Returns
///
/// An `impl Responder` representing the HTTP response.
pub async fn get_block(
    bc: web::Data<Mutex<Blockchain>>,
    hash: web::Path<String>,
) -> impl Responder {
    let bc_ref = bc.lock().unwrap();

    for block in &bc_ref.blocks {
        if block.hash == hash.to_string() {
            return HttpResponse::Ok().json(block);
        }
    }

    HttpResponse::NotFound().body("Block not found")
}


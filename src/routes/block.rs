use crate::blockchain::Blockchain;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;

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

use crate::blockchain::Blockchain;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;

/// Retrieves the entire blockchain.
///
/// # Arguments
///
/// * `bc` - The shared mutable state of the blockchain wrapped in a Mutex.
///
/// # Returns
///
/// An `impl Responder` representing the HTTP response.
pub async fn index(bc: web::Data<Mutex<Blockchain>>) -> impl Responder {
    let bc_ref = bc.lock().unwrap();
    HttpResponse::Ok().json(&*bc_ref)
}


use crate::blockchain::Blockchain;
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use std::sync::Mutex;

/// Represents the response for retrieving the balance of an address.
#[derive(Serialize)]
struct BalanceResponse {
    /// The balance of the address.
    balance: u32,
}

/// Retrieves the balance of an address from the blockchain.
///
/// # Arguments
///
/// * `bc_data` - The shared mutable state of the blockchain wrapped in a Mutex.
/// * `address` - The address for which to retrieve the balance.
///
/// # Returns
///
/// An `impl Responder` representing the HTTP response.
pub async fn get_balance(
    bc_data: web::Data<Mutex<Blockchain>>,
    address: web::Path<String>,
) -> impl Responder {
    let bc = bc_data.lock().unwrap();

    let mut balance = 0;
    for block in &bc.blocks {
        for tx in block.data.iter() {
            if tx.sender == address.to_string() {
                balance -= tx.amount;
            }
            if tx.receiver == address.to_string() {
                balance += tx.amount;
            }
        }
    }

    HttpResponse::Ok().json(BalanceResponse { balance })
}


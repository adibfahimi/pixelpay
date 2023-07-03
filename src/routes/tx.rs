use crate::{blockchain::Blockchain, tx::Tx};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

/// Represents the response for adding a transaction.
#[derive(Debug, Serialize, Deserialize)]
struct TxResponse {
    /// The status of the transaction.
    status: String,
}

/// Adds a transaction to the blockchain.
///
/// # Arguments
///
/// * `bc` - The shared mutable state of the blockchain wrapped in a Mutex.
/// * `tx` - The JSON payload containing the transaction data.
///
/// # Returns
///
/// An `impl Responder` representing the HTTP response.
pub async fn add_tx(bc: web::Data<Mutex<Blockchain>>, tx: web::Json<Tx>) -> impl Responder {
    let mut bc = bc.lock().unwrap();

    match tx.validate() {
        Ok(_) => {
            bc.pending_txs.push(tx.into_inner());
            HttpResponse::Ok().json(TxResponse {
                status: "success".to_string(),
            })
        }
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}


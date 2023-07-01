use std::sync::Mutex;

use crate::{blockchain::Blockchain, tx::Tx};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TxResponse {
    status: String,
}

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

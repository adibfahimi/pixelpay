use std::sync::Mutex;

use crate::{blockchain::Blockchain, tx::Tx};
use actix_web::{web, HttpResponse, Responder};

pub async fn add_tx(bc: web::Data<Mutex<Blockchain>>, tx: web::Json<Tx>) -> impl Responder {
    let mut bc = bc.lock().unwrap();

    match tx.validate() {
        Ok(_) => {
            bc.pending_txs.push(tx.into_inner());
            HttpResponse::Ok().body("Transaction added")
        }
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

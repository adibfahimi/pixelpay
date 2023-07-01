use crate::blockchain::Blockchain;
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use std::sync::Mutex;

#[derive(Serialize)]
struct BalanceResponse {
    balance: u32,
}

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

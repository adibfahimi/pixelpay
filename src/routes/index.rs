use std::sync::Mutex;

use crate::blockchain::Blockchain;
use actix_web::{web, HttpResponse, Responder};

pub async fn index(bc: web::Data<Mutex<Blockchain>>) -> impl Responder {
    let bc_ref = bc.lock().unwrap();
    HttpResponse::Ok().json(&*bc_ref)
}


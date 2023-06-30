use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
mod block;
mod blockchain;
mod routes;
mod tx;

use blockchain::Blockchain;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let bc = Blockchain::new();

    let bc_data = web::Data::new(Mutex::new(bc));

    HttpServer::new(move || {
        App::new()
            .app_data(bc_data.clone())
            .route("/", web::get().to(routes::index::index))
            .route("/mine", web::get().to(routes::mine::get_block))
            .route("/mine", web::post().to(routes::mine::add_block))
            .route("/tx", web::post().to(routes::tx::add_tx))
            .route("/block/{hash}", web::get().to(routes::block::get_block))
            .route(
                "/balance/{address}",
                web::get().to(routes::balance::get_balance),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

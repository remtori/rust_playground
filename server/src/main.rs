use std::sync::{atomic::AtomicUsize, Arc};

use actix::*;
use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};

mod logger;
mod server;
mod ws_session;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::setup_logger().unwrap();

    let app_state = Arc::new(AtomicUsize::new(0));

    let server = server::GameServer::new(app_state.clone()).start();

    HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            .data(server.clone())
            .wrap(Logger::default())
            // websocket
            .route("/ws/", web::get().to(ws_session::route))
            // static resources
            .service(fs::Files::new("/", "./client").index_file("index.html"))
    })
    .bind("127.0.0.1:8008")?
    .run()
    .await
}

#![feature(try_blocks)]

#[forbid(unsafe_code, unused_imports, incomplete_features)]
mod actors;
mod routes;
mod messages;
mod tls;
mod graphql;
mod storage;

use actix_web::{App, HttpServer};

use crate::tls::load_ssl;

use crate::routes::set_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_ssl();
    HttpServer::new(move || App::new()
        .service(actix_files::Files::new("/sources", "sources")
            .prefer_utf8(true))
        .configure(set_config))
        .bind_rustls("192.168.0.7:8081", config)?
        .run()
        .await
}

#![feature(try_blocks)]
#![feature(format_args_capture)]
#![feature(generic_associated_types)]
#![feature(associated_type_bounds)]

#[forbid(unsafe_code, incomplete_features)]
mod actors;
mod extensions;
mod graphql;
mod messages;
mod proto;
mod routes;
mod storage;
mod tls;

use actix_web::{ web, App, HttpServer, Result};

use crate::routes::set_config;
use crate::tls::load_ssl;

async fn index() -> Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_ssl();
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/sources", "sources").prefer_utf8(true))
            .service(actix_files::Files::new("/", ".dist").prefer_utf8(true))
            .default_service(
                web::resource("/")
                    .route(web::get().to(index)))
            .configure(set_config)
    })
    .bind_rustls("192.168.0.7:8081", config)?
    .run()
    .await
}

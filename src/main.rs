#![feature(try_blocks)]
#![feature(format_args_capture)]
#![feature(generic_associated_types)]
#![feature(associated_type_bounds)]

#[forbid(unsafe_code, incomplete_features)]
mod actors;
mod routes;
mod messages;
mod tls;
mod graphql;
mod storage;
mod proto;
mod extensions;

use actix_web::{App, HttpServer, Result, web};

use crate::tls::load_ssl;

use crate::routes::set_config;
use actix_cors::Cors;


async fn index() -> Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("dist/index.html")?)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix-files=debug");
    let config = load_ssl();
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(cors)
            .service(actix_files::Files::new("/sources", "sources")
                .prefer_utf8(true))
            .service(actix_files::Files::new("/dist", "./dist/")
                .prefer_utf8(true)
            )
            .default_service(
                web::resource("/")
                    .route(web::get().to(index))
            )
            .configure(set_config)
    })
        .bind_rustls("192.168.0.7:8081", config)?
        .run()
        .await
}

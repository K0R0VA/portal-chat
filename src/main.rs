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

use actix_web::{App, HttpServer, Result, get, web};

use crate::tls::load_ssl;

use crate::routes::set_config;
use actix_web::web::Path;

async fn index() -> Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("static/index.html")?)
}
async fn worker() -> Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("static/ngsw-worker.js")?)
}

#[get("assets/data/{path}")]
async fn assets(path: Path<String>) -> Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open(&format!("static/assets/data/{}", path.into_inner()))?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_ssl();
    HttpServer::new(move || App::new()
        .service(actix_files::Files::new("/sources", "sources")
            .prefer_utf8(true))
        .service(actix_files::Files::new("/static", "static")
            .prefer_utf8(true))
        .service(web::resource("ngsw-worker.js").route(web::get().to(worker)))
        .default_service(
            web::resource("")
                .route(web::get().to(index))
        )
        .service(assets)
        .configure(set_config))
        .bind_rustls("192.168.0.7:8081", config)?
        .run()
        .await
}

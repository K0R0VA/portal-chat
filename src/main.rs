mod actors;
mod routes;

use actix_web::{App, HttpServer, Result, HttpResponse, http};
use actix_web::web;
use actix::Actor;
use actix_files as fs;
use crate::routes::start_connection;
use crate::actors::lobby::Lobby;
use actix_web::dev::{Service};
use futures::prelude::future::ok;
use futures::future::Either;
use rustls::{ServerConfig, NoClientAuth};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};


fn load_ssl() -> ServerConfig {
    use std::io::BufReader;

    const CERT: &'static [u8] = include_bytes!("../192.168.0.7.pem");
    const KEY: &'static [u8] = include_bytes!("../192.168.0.7-key.pem");

    let mut cert = BufReader::new(CERT);
    let mut key = BufReader::new(KEY);

    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_chain = certs(&mut cert).unwrap();
    let mut keys = pkcs8_private_keys(&mut key).unwrap();
    config
        .set_single_cert(cert_chain, keys.remove(0)).unwrap();

    config
}

async fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = Lobby::default().start();
    let config = load_ssl();
    HttpServer::new(move || App::new()
        .data(chat_server.clone())
        .wrap_fn(|req, res| {
            if req.connection_info().scheme() == "http" {
                let host = req.connection_info().host().to_owned();
                let uri = req.uri().to_owned();
                let url = format!("https://{}{}", host, uri);
                return Either::Left(ok(req.into_response(HttpResponse::TemporaryRedirect()
                    .header(http::header::LOCATION, url)
                    .finish())));
            }
            Either::Right(res.call(req))
        })
        .service(start_connection)
        .service(fs::Files::new("/static", "static")
            .prefer_utf8(true))
        .default_service(
            web::resource("")
                .route(web::get().to(index))
        ))
        .bind_rustls("192.168.0.7:8080", config)?
        .run()
        .await
}

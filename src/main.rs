mod actors;
mod routes;

use actix_web::{App, HttpServer};
use crate::actors::lobby::Lobby;
use actix::Actor;
use crate::routes::start_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = Lobby::default().start();
    HttpServer::new(move || App::new().data(chat_server.clone()).service(start_connection))
        .bind("192.168.0.7:8080")?
        .run()
        .await
}

use actix_web::{get, HttpRequest, HttpResponse, Error};
use actix_web::web::{Payload, Path, Data};
use actix_web_actors::ws::start;
use actix::Addr;
use uuid::Uuid;

use crate::actors::lobby::Lobby;
use crate::actors::web_socket::WebSocket;

#[get("start/{group_id}")]
pub async fn start_connection (
    request: HttpRequest,
    stream: Payload,
    group_id: Path<Uuid>,
    service: Data<Addr<Lobby>>) -> Result<HttpResponse, Error> {
    let web_socket = WebSocket::new(group_id.into_inner(), service.get_ref().clone());
    let response = start(web_socket, &request, stream)?;
    Ok(response)
}
use actix_web::{get, post, guard, HttpResponse, HttpRequest, Result};
use actix_web::web::{Payload, Data, ServiceConfig};
use async_graphql_actix_web::{Request, Response};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

use crate::graphql::{create_schema, DefaultSchema};
use crate::actors::web_socket::WebSocket;
use actix::{Addr, Actor};
use crate::actors::state::State;

pub fn set_config(config: &mut ServiceConfig) {
    let chat_server = State::default().start();
    let schema = create_schema(chat_server.clone());
    config
        .data(chat_server)
        .data(schema)
        .service(
            actix_web::web::resource("")
                .guard(guard::Get())
                .to(start)
        )
        .service(graphql)
        .service(graphiql);
}


#[get("/")]
async fn graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    )
}

#[post("/")]
async fn graphql(schema: Data<DefaultSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn start(state: Data<Addr<State>>, req: HttpRequest, payload: Payload) -> Result<HttpResponse> {
    actix_web_actors::ws::start(WebSocket::new(state.get_ref().clone()), &req, payload)?;
    Ok(HttpResponse::NoContent().finish())
}
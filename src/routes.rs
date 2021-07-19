use actix_web::{get, post, HttpResponse, HttpRequest, Result};
use actix_web::web::{Payload, Data, ServiceConfig};
use async_graphql_actix_web::{Request, Response};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use actix::{Addr, Actor};
use async_graphql::{Schema, EmptySubscription};


use crate::graphql::{DefaultSchema};
use crate::actors::web_socket::WebSocket;
use crate::actors::state::State;
use crate::graphql::query::Query;
use crate::graphql::mutation::Mutation;
use crate::storage::Storage;

pub fn set_config(config: &mut ServiceConfig) {
    let chat_server = State::default().start();
    let storage = Storage::default();
    let storage_actor = storage.clone().start();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(storage_actor.clone())
        .data(chat_server.clone())
        .finish();
    config
        .data(chat_server)
        .data(schema)
        .data(storage_actor)
        .service(start)
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

#[get("/start")]
async fn start(state: Data<Addr<State>>, req: HttpRequest, payload: Payload) -> Result<HttpResponse> {
    let response = actix_web_actors::ws::start(WebSocket::new(state.get_ref().clone()), &req, payload)?;
    Ok(response)
}
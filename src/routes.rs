use actix_web::{get, post, HttpResponse, HttpRequest, Result, web, guard};
use actix_web::web::{Data, ServiceConfig, Payload, Path};
use async_graphql_actix_web::{Request, Response, WSSubscription};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use actix::{Actor, Addr};
use async_graphql::{Schema, EmptySubscription};


use crate::graphql::{DefaultSchema};
use crate::actors::state::State;
use crate::graphql::query::Query;
use crate::graphql::mutation::Mutation;
use crate::storage::Storage;
use crate::actors::file_writer::FileWriter;
use crate::actors::web_socket::WebSocket;

pub fn set_config(config: &mut ServiceConfig) {
    let chat_server = State::default().start();
    let file_server = FileWriter.start();
    let storage = Storage::default();
    let storage_actor = storage.clone().start();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(storage_actor.clone())
        .data(chat_server.clone())
        .data(file_server)
        .finish();
    config
        .data(chat_server)
        .data(schema)
        .data(storage_actor)
        .service(start)
        .service(graphql)
        .service(graphiql);
}


#[get("/graphql")]
async fn graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")
            .subscription_endpoint("/start")
        ))
    )
}

#[post("/graphql")]
async fn graphql(schema: Data<DefaultSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

#[get("/start/{user_id}")]
async fn start(state: Data<Addr<State>>, req: HttpRequest, payload: Payload, user_id: Path<i32>) -> Result<HttpResponse> {
    let client = WebSocket::new(state.get_ref().clone(), *user_id);
    let response = actix_web_actors::ws::start(client, &req, payload)?;
    Ok(response)
}
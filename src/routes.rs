use actix_web::{get, post, HttpResponse, HttpRequest, Result};
use actix_web::web::{Data, ServiceConfig, Payload, Path};
use async_graphql_actix_web::{Request, Response};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use actix::{Actor, Addr};
use async_graphql::{Schema, EmptySubscription};


use crate::graphql::{DefaultSchema};
use crate::actors::state::ChatState;
use crate::graphql::query::Query;
use crate::graphql::mutation::Mutation;
use crate::actors::file_writer::FileWriter;
use crate::storage::get_pool;
use crate::actors::mutation_handlers::setup_handlers;
use crate::graphql::loaders::setup_loaders;
use crate::messages::ws_messages::Connect;

pub fn set_config(config: &mut ServiceConfig) {
    if let Ok(pool) = get_pool().map_err(|e| print!("{}", e)) {
        let chat_server = ChatState::default().start();
        let file_server = FileWriter.start();
        let builder = Schema::build(Query, Mutation, EmptySubscription);
        let builder = setup_handlers(builder, pool.clone());
        let builder = setup_loaders(builder, pool);
        let schema = builder
            .data(file_server)
            .data(chat_server.clone())
            .finish();
        config
            .data(chat_server)
            .data(schema)
            .service(start)
            .service(graphql)
            .service(graphiql);
    }
}


#[get("/graphql")]
async fn graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    )
}

#[post("/graphql")]
async fn graphql(schema: Data<DefaultSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

#[get("/start/{user_id}")]
async fn start(
    state: Data<Addr<ChatState>>,
    req: HttpRequest,
    payload: Payload,
    Path(user_id): Path<i32>)
    -> Result<HttpResponse> {
    if let Ok(session) = state.send(Connect { user_id }).await {
        return actix_web_actors::ws::start(session, &req, payload)
    };
    HttpResponse::BadGateway().await
}


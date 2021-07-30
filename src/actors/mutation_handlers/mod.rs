use deadpool_postgres::Pool;
use actix::Actor;

use crate::actors::mutation_handlers::logout_handler::LogoutHandler;
use crate::graphql::Builder;

pub mod sign_in_handler;
pub mod sign_up_handler;
pub mod logout_handler;
pub mod room_creator_handler;

pub fn setup_handlers(builder: Builder, pool: Pool) -> Builder {
    let logout_handler = LogoutHandler {pool: pool.clone()}.start();
    let sign_in_handler = LogoutHandler {pool: pool.clone()}.start();
    let sign_up_handler = LogoutHandler {pool}.start();
    builder
        .data(logout_handler)
        .data(sign_in_handler)
        .data(sign_up_handler)

}
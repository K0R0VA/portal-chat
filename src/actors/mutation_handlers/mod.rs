use deadpool_postgres::Pool;
use actix::Actor;

use crate::actors::mutation_handlers::logout_handler::LogoutHandler;
use crate::graphql::Builder;
use crate::actors::mutation_handlers::room_creator_handler::RoomCreatorHandler;
use crate::actors::mutation_handlers::add_contact_handler::AddContactHandler;

pub mod sign_in_handler;
pub mod sign_up_handler;
pub mod logout_handler;
pub mod room_creator_handler;
pub mod add_contact_handler;

pub fn setup_handlers(builder: Builder, pool: Pool) -> Builder {
    let logout_handler = LogoutHandler { pool: pool.clone() }.start();
    let sign_in_handler = LogoutHandler { pool: pool.clone() }.start();
    let room_creator = RoomCreatorHandler { pool: pool.clone() }.start();
    let add_contact_handler = AddContactHandler { pool: pool.clone() }.start();
    let sign_up_handler = LogoutHandler { pool }.start();
    builder
        .data(add_contact_handler)
        .data(room_creator)
        .data(logout_handler)
        .data(sign_in_handler)
        .data(sign_up_handler)
}
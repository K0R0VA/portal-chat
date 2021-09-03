use actix::{Actor, AsyncContext, ActorContext};
use actix_web_actors::ws::WebsocketContext;
use std::time::{Instant, Duration};

pub mod state;
pub mod session;
pub mod room;
pub mod file_writer;
pub mod mutation_handlers;
pub mod user;
pub mod state_storage;
pub mod private_streaming_lobby;
pub mod streaming_session;

trait Session: Actor<Context = WebsocketContext<Self>> {

    const HEARTBEAT_INTERVAL: Duration;
    const CLIENT_TIMEOUT: Duration;

    fn get_heartbeat(&self) -> Instant;

    fn init_heartbeat(&self, ctx: &mut WebsocketContext<Self>) {
        ctx.run_interval(Self::HEARTBEAT_INTERVAL, |actor, ctx| {
            if Instant::now().duration_since(actor.get_heartbeat()) > Self::CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
            ctx.ping(b"PING");
        });
    }
}
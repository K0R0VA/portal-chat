use deadpool_postgres::Pool;
use actix::{Actor, Context, Handler};
use actix::dev::MessageResponse;

pub struct Database {
    pool: Pool
}

impl Actor for Database {
    type Context = Context<Self>;

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.pool.close();
    }
}


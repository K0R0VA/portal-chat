use actix::{Actor, Context, Handler, ResponseFuture};
use deadpool_postgres::Client;

use crate::storage::Storage;
use crate::messages::db_messages::{SignUp};


impl Actor for Storage {
    type Context = Context<Self>;
}

impl Handler<SignUp> for Storage {
    type Result = ResponseFuture<i32>;

    fn handle(&mut self, msg: SignUp, _ctx: &mut Self::Context) -> Self::Result {
        let pool = self.pool.clone();
        Box::pin(async move {
            let client: Client = pool.get().await.unwrap();
            let stmt = client
                .prepare(r#"insert into "User" (name) values ($1) RETURNING id"#)
                .await
                .unwrap();
            let result = client
                .query_one(&stmt, &[&msg.name])
                .await
                .unwrap();
            result.get(0)
        })
    }
}
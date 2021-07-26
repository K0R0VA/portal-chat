use deadpool_postgres::{Pool, Client};
use actix::{Actor, Context, Handler, ResponseFuture};
use crate::messages::db_messages::Logout;
use chrono::Utc;

pub struct LogoutHandler {
    pub(crate) pool: Pool,
}

impl Actor for LogoutHandler {
    type Context = Context<Self>;
}

impl Handler<Logout> for LogoutHandler {
    type Result = ResponseFuture<anyhow::Result<()>>;

    fn handle(&mut self, msg: Logout, _ctx: &mut Self::Context) -> Self::Result {
        let pool = self.pool.clone();
        Box::pin(async move {
            try {
                let client: Client = pool.get().await?;
                let this_moment = Utc::now().naive_utc();
                let stmt = client
                    .prepare(r#"update public.user set last_session = $1 where id = $2"#)
                    .await?;
                client.execute(&stmt, &[&this_moment, &msg.user_id])
                    .await?;
            }
        })
    }
}
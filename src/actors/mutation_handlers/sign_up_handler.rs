use deadpool_postgres::{Pool, Client};
use actix::{Actor, Context, Handler, ResponseFuture};
use chrono::Utc;

use crate::messages::db_messages::SignUp;
use crate::graphql::models::user::User;

pub struct SignUpHandler {
    pub(crate) pool: Pool,
}

impl Actor for SignUpHandler {
    type Context = Context<Self>;
}


impl Handler<SignUp> for SignUpHandler {
    type Result = ResponseFuture<anyhow::Result<User>>;

    fn handle(&mut self, msg: SignUp, _ctx: &mut Self::Context) -> Self::Result {
        let pool = self.pool.clone();
        Box::pin(async move {
            try {
                let client: Client = pool.get().await?;
                let this_moment = Utc::now().naive_utc();
                let stmt = client
                    .prepare(r#"insert into public.user (name, password, last_sesstion) values ($1, $2, $3) RETURNING *"#)
                    .await?;
                let result = client
                    .query_one(&stmt, &[&msg.name, &msg.password, &this_moment])
                    .await?;
                let user = serde_postgres::from_row::<User>(&result)?;
                user
            }
        })
    }
}
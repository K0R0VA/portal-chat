use deadpool_postgres::{Pool, Client};
use actix::{Handler, ResponseFuture, Actor, Context};
use chrono::Utc;

use crate::messages::db_messages::SignIn;
use crate::graphql::models::user::User;

pub struct SignInHandler {
    pub(crate) pool: Pool,
}

impl Actor for SignInHandler {
    type Context = Context<Self>;
}

impl Handler<SignIn> for SignInHandler {
    type Result = ResponseFuture<anyhow::Result<User>>;

    fn handle(&mut self, msg: SignIn, _ctx: &mut Self::Context) -> Self::Result {
        let pool = self.pool.clone();
        Box::pin(async move {
            try {
                let client: Client = pool.get().await?;
                let this_moment = Utc::now().naive_utc();
                let stmt = client
                    .prepare(r#"update public.user set last_session = $1 where name = $2 and password = $3 return *"#)
                    .await?;
                let result = client
                    .query_one(&stmt, &[&this_moment, &msg.name, &msg.password])
                    .await?;
                let user = serde_postgres::from_row::<User>(&result)?;
                user
            }
        })
    }
}


use actix::{Actor, Context, Handler, ResponseFuture};
use deadpool_postgres::Client;

use crate::storage::Storage;
use crate::messages::db_messages::{SignUp, SignIn, Logout};
use chrono::{Utc};
use crate::graphql::models::user::User;


impl Actor for Storage {
    type Context = Context<Self>;
}

impl Handler<SignUp> for Storage {
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

impl Handler<SignIn> for Storage {
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

impl Handler<Logout> for Storage {
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
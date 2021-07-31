use deadpool_postgres::Pool;
use actix::{Actor, Context, Handler, ResponseFuture};
use crate::messages::db_messages::AddContact;
use crate::graphql::models::user::User;

pub struct AddContactHandler {
    pub pool: Pool
}

impl Actor for AddContactHandler {
    type Context = Context<Self>;
}

impl Handler<AddContact> for AddContactHandler {
    type Result = ResponseFuture<anyhow::Result<User>>;

    fn handle(&mut self, msg: AddContact, _: &mut Self::Context) -> Self::Result {
        let pool = self.pool.clone();
        Box::pin(async move {
            try {
                let client = pool.get().await?;
                let stmt = client
                    .prepare("with insert_statement as (insert into contact (user_id, contact_id)
						                values ($1, $2) returning contact_id)
                                    select * from public.user where id = (select contact_id from insert_statement);"
                    ).await?;
                let row = client
                    .query_one(&stmt, &[&msg.user_id, &msg.contact_id]).await?;
                serde_postgres::from_row::<User>(&row)?
            }
        })
    }
}
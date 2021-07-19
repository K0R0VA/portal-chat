use deadpool_postgres::{Pool, Client};
use async_graphql::dataloader::Loader;
use std::collections::HashMap;
use itertools::join;

use crate::graphql::loaders::LoadResult;
use crate::graphql::models::user::{UserId, User};

pub struct UserLoader(Pool);

#[async_trait::async_trait]
impl Loader<UserId> for UserLoader {
    type Value = User;
    type Error = String;

    async fn load(&self, keys: &[UserId]) -> Result<HashMap<UserId, Self::Value>, Self::Error> {
        let result: LoadResult<UserId, Self::Value> = try {
            let pool = &self.0;
            let client: Client = pool.get().await?;
            let fields = keys[0].1.as_str();
            let stmt = client.prepare(
                &*format!(
                    r#"select id, {} from public.user where id in ({})"#,
                    fields,
                    join(keys
                             .into_iter()
                             .map::<i32, _>(|user_id| user_id.0),
                         ",")
                )
            ).await?;
            let result = client.query(&stmt, &[]).await?;
            let users = result.iter().map(|row| {
                let user: User = serde_postgres::from_row(row).unwrap();
                (
                    UserId(user.id, fields.to_string()),
                    user
                )
            }).collect();
            users
        };
        result.map_err(|e| e.to_string())
    }
}



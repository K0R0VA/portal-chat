use deadpool_postgres::{Client};
use async_graphql::dataloader::*;
use std::collections::HashMap;
use itertools::join;

use crate::graphql::models::{UserId, User};
use crate::storage::Storage;

#[async_trait::async_trait]
impl Loader<UserId> for Storage {
    type Value = User;
    type Error = tokio_postgres::error::DbError;

    async fn load(&self, keys: &[UserId]) -> Result<HashMap<UserId, Self::Value>, Self::Error> {
        let pool = self.pool.clone();
        let client: Client = pool.get().await.unwrap();
        let stmt = client.prepare_cached(
            &*format!("select * from User where id in ({})",
                      join(keys
                               .into_iter()
                               .map::<i32, _>(|user_id| **user_id),
                           ",")
            )
        ).await.unwrap();
        let result = client.query(&stmt, &[]).await.unwrap();
        Ok(result
            .into_iter()
            .map(|row|
                (
                    UserId(row.get(0)),
                    User
                    {
                        id: row.get(0),
                        name: row.get(1),
                    }))
            .collect::<HashMap<UserId, User>>()
        )
    }
}


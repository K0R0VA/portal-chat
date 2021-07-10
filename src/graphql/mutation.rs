use async_graphql::{Object, Context, Result};

pub struct Mutation;

#[Object]
impl Mutation {
    async fn sign_up(&self, _ctx: &Context<'_>) -> Result<bool> {
        Ok(true)
    }
}
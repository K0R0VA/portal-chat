use async_graphql::Object;

pub struct Query;

#[Object]
impl Query {
    async fn api_version(&self) -> &str {
        "1.0"
    }
}
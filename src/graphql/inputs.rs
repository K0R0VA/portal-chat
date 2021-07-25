use async_graphql::InputObject;

#[derive(InputObject)]
pub struct Credentials {
    pub name: String,
    pub password: String,
}
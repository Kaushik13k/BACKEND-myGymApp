use super::users::User;
use juniper::GraphQLObject;

#[derive(GraphQLObject)]

pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct Login {
    pub username: String,
    pub password: String,
}

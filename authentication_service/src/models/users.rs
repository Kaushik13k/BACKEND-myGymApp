use diesel::prelude::Queryable;

#[derive(Queryable, Debug, juniper::GraphQLObject)]

pub struct User {
    pub id: i32,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: String,
    pub email: String,
    pub hash: Option<String>,
    pub phone_number: Option<i32>,
    pub dob: Option<String>,
}

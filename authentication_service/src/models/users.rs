use diesel::prelude::Queryable;

#[derive(Queryable, Debug, juniper::GraphQLObject)]

pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub hash: String,
    pub phone_number: i32,
    pub age: i32,
}

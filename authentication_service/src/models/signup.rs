use crate::schema::users;
use diesel::Insertable;

#[derive(Insertable)]
#[diesel(table_name = users::users)]

pub struct Signup<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub username: &'a str,
    pub email: &'a str,
    pub hash: &'a str,
    pub phone_number: &'a i32,
    pub age: &'a i32,
}

#[derive(juniper::GraphQLInputObject)]
pub struct InputSignup {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone_number: i32,
    pub age: i32,
}

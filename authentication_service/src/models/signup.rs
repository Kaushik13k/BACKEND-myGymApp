use crate::schema::users;
use diesel::Insertable;

#[derive(Insertable, Debug)]
#[diesel(table_name = users::users)]

pub struct Signup {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: String,
    pub email: String,
    pub hash: String,
    pub phone_number: Option<i32>,
    pub age: Option<i32>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct InputSignup {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone_number: Option<i32>,
    pub age: Option<i32>,
}

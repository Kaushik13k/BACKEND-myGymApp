use crate::schema::users;
use diesel::Insertable;

#[derive(Insertable, Debug)]
#[diesel(table_name = users::users)]

pub struct Profile {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub phone_number: i32,
    pub dob: i32,
}

#[derive(juniper::GraphQLInputObject)]
pub struct InputProfile {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub phone_number: i32,
    pub dob: i32,
}

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

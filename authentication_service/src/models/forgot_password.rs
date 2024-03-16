use crate::schema::users;
use diesel::Insertable;

#[derive(Insertable)]
#[diesel(table_name = users::users)]

pub struct ForgotPass<'a> {
    pub username: &'a str,
    pub hash: &'a str,
}

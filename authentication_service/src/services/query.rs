use crate::database::connection::Context;
use crate::models::users::User;
use crate::services::login;

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    pub fn login(&self, context: &Context, username: String, email: String) -> User {
        login::user_login(context, username, email)
    }
}

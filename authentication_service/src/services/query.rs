use crate::database::connection::Context;
use crate::models::users::User;
use crate::services::login;
use juniper::FieldError;

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    pub fn login(
        &self,
        context: &Context,
        username: String,
        password: String,
    ) -> Result<User, FieldError> {
        login::user_login(context, username, password)
    }
}

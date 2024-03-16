use crate::database::connection::Context;
use crate::models::users::User;
use crate::services::get_user;
use crate::services::login;
use crate::services::token;
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

    pub fn token(&self, context: &Context, username: String) -> Result<String, FieldError> {
        token::get_token(context, username)
    }

    pub fn user(&self, context: &Context, username: String) -> Result<bool, FieldError> {
        get_user::get_availablity(context, username)
    }
}

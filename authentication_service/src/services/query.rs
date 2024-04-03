use crate::database::connection::Context;
use crate::models::body_measurements::UserBodyMeasurements;
use crate::models::login::Login;
use crate::models::login::LoginResponse;
use crate::models::user_inputs::UserAvailable;
use crate::services::get_user;
use crate::services::login;
use crate::services::token;
use juniper::FieldError;

use super::get_body_measurements;

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    pub fn login(&self, context: &Context, user_input: Login) -> Result<LoginResponse, FieldError> {
        login::user_login(context, user_input.username, user_input.password)
    }

    pub fn token(
        &self,
        context: &Context,
        user_input: UserAvailable,
    ) -> Result<String, FieldError> {
        token::get_token(context, user_input.username)
    }

    pub fn user(&self, context: &Context, user_input: UserAvailable) -> Result<bool, FieldError> {
        get_user::get_availablity(context, user_input.username)
    }

    pub fn user_body_measurements(
        &self,
        context: &Context,
        user_input: UserAvailable,
    ) -> Result<UserBodyMeasurements, FieldError> {
        get_body_measurements::body_measurements(context, user_input.username)
    }
}

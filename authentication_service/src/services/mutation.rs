use crate::database::connection::Context;
use crate::models::body_measurements::BodyMeasurementsInput;
use crate::models::body_measurements::BodyMeasurementsResult;
use crate::models::signup::InputSignup;
use crate::models::user_inputs::ForgotPassword;
use crate::models::users::User;
use crate::services::forgot_password;
use crate::services::signup;
use juniper::FieldError;

use super::insert_body_measurements;

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    pub fn signup(&self, context: &Context, user_input: InputSignup) -> Result<User, FieldError> {
        signup::user_signup(context, user_input)
    }
    pub fn forgot_password(
        &self,
        context: &Context,
        user_input: ForgotPassword,
    ) -> Result<User, FieldError> {
        forgot_password::forgot(context, user_input)
    }
    pub fn body_measurements(
        &self,
        context: &Context,
        user_input: BodyMeasurementsInput,
    ) -> Result<BodyMeasurementsResult, FieldError> {
        insert_body_measurements::insert_body_measurements(context, user_input)
    }
}

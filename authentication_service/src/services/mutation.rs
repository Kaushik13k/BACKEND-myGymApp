use crate::database::connection::Context;
use crate::models::user_inputs::InputSignup;
use crate::models::users::User;
use crate::services::signup;
use juniper::FieldError;

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    pub fn signup(&self, context: &Context, user_input: InputSignup) -> Result<User, FieldError> {
        signup::user_signup(context, user_input)
    }
}

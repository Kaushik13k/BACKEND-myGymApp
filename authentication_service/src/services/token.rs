use crate::services::get_user;
use crate::{database::connection::Context, utils::token};
use juniper::FieldError;
use log::info;

pub fn get_token(context: &Context, username: String) -> Result<String, FieldError> {
    let user_data = get_user::get_user(context, username);
    info!("User found: {:?}", user_data);
    match user_data {
        Ok(user_data) => {
            info!("The query resulted with some data.");
            let token =
                token::encode_token(user_data.email.to_owned(), user_data.username.to_owned());
            info!("Token: {:?}", token);
            Ok(token)
        }
        Err(diesel::result::Error::NotFound) => {
            info!("No user found");
            Err(FieldError::new("No user found", juniper::Value::null()))
        }
        Err(e) => {
            info!("There was an error in the while getting token: {:?}", e);
            Err(FieldError::new(
                format!("Error: {:?}", e),
                juniper::Value::null(),
            ))
        }
    }
}

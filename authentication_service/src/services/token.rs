use crate::services::get_user;
use crate::{database::connection::Context, utils::token};
use juniper::FieldError;
use log::{error, info};

pub fn get_token(context: &Context, username: String) -> Result<String, FieldError> {
    info!("Getting token for: {:?}", username);
    let user_data = get_user::get_user(context, username);

    match user_data {
        Ok(user_data) => {
            info!("User found: {:?}", user_data);
            let token =
                token::encode_token(user_data.email.to_owned(), user_data.username.to_owned());
            Ok(token)
        }
        Err(diesel::result::Error::NotFound) => {
            error!("No user found");
            Err(FieldError::new("No user found", juniper::Value::null()))
        }
        Err(e) => {
            error!("There was an error in the while getting token: {:?}", e);
            Err(FieldError::new(
                format!("Error: {:?}", e),
                juniper::Value::null(),
            ))
        }
    }
}

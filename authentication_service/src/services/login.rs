use crate::database::connection::Context;
use crate::models::users::User;
use crate::services::get_user;
use crate::utils::helpers;
use juniper::FieldError;
use log::{error, info};

pub fn user_login(
    context: &Context,
    username: String,
    password: String,
) -> Result<User, FieldError> {
    info!("User login: {:?}", username);
    let user_data = get_user::get_user(context, username);
    match user_data {
        Ok(user_data) => {
            info!("User found...");
            let is_valid_password = helpers::verify_password(&password, &user_data.hash);
            info!("User is_valid_password: {:?}", is_valid_password);
            if is_valid_password.unwrap() {
                info!("User is valid");
                Ok(user_data)
            } else {
                error!("Invalid Credentials");
                Err(FieldError::new(
                    "Invalid Credentials",
                    juniper::Value::null(),
                ))
            }
        }
        Err(diesel::result::Error::NotFound) => {
            error!("No user found");
            Err(FieldError::new("No user found", juniper::Value::null()))
        }
        Err(e) => {
            error!("There was an error in the login process: {:?}", e);
            Err(FieldError::new(
                format!("Error: {:?}", e),
                juniper::Value::null(),
            ))
        }
    }
}

use crate::models::login::LoginResponse;
use crate::services::get_user;
use crate::utils::helpers;
use crate::{database::connection::Context, utils::token::encode_token};
use juniper::FieldError;
use log::{error, info};

pub fn user_login(
    context: &Context,
    username: String,
    password: String,
) -> Result<LoginResponse, FieldError> {
    info!("User login: {:?}", username);
    let user_data = get_user::get_user(context, username);
    match user_data {
        Ok(user_data) => {
            info!("User found...");
            let is_valid_password =
                helpers::verify_password(&password, &user_data.hash.as_ref().unwrap().to_string());
            info!("User is_valid_password: {:?}", is_valid_password);
            if is_valid_password.unwrap() {
                info!("User is valid");
                let token =
                    encode_token(user_data.email.to_string(), user_data.username.to_string());
                let response = LoginResponse {
                    user: user_data,
                    token,
                };
                // let response = LoginResponse {
                //     id: user_data.id,
                //     firstname: user_data.firstname,
                //     lastname: user_data.lastname,
                //     username: user_data.username,
                //     email: user_data.email,
                //     hash: user_data.hash,
                //     phone_number: user_data.phone_number,
                //     age: user_data.age,
                //     token,
                // };
                Ok(response)
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

use crate::{
    database::connection::Context,
    models::{forgot_password::ForgotPass, user_inputs::ForgotPassword, users::User},
    schema::users,
    utils::helpers::hash_password,
};
use diesel::prelude::*;
use juniper::FieldError;
use log::{error, info};

use super::get_user;

pub fn update_user(context: &Context, new_user: ForgotPass) -> Result<User, diesel::result::Error> {
    info!("Inserting user");
    let connection = &mut context.db.establish_connection();
    let filter = users::users::table.filter(users::users::username.eq(new_user.username));

    let updated_user = diesel::update(filter)
        .set(users::users::hash.eq(new_user.hash))
        .get_result(connection);
    return updated_user;
}

pub fn forgot(context: &Context, user_input: ForgotPassword) -> Result<User, FieldError> {
    info!("Reset Password: {:?}", user_input.username);
    let username = user_input.username.clone();
    let password_hashed = hash_password(&user_input.password);
    let user_data = get_user::get_user(context, username);

    match user_data {
        Ok(_) => {
            info!("User exists");
            let new_user = ForgotPass {
                username: &user_input.username,
                hash: &password_hashed.unwrap(),
            };
            match update_user(context, new_user) {
                Ok(user) => {
                    info!("The user password updated successfully.");
                    Ok(user)
                }
                Err(_) => {
                    error!("Failed to update password");
                    Err(FieldError::new(
                        "Failed to update password",
                        juniper::Value::null(),
                    ))
                }
            }
        }
        Err(diesel::result::Error::NotFound) => {
            info!("No user found");
            return Err(FieldError::new("No user found", juniper::Value::null()));
        }
        Err(e) => {
            error!("There was an error in the while updating password: {:?}", e);
            return Err(FieldError::new(
                format!("Error: {:?}", e),
                juniper::Value::null(),
            ));
        }
    }
}

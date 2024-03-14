use crate::database::connection::Context;
use crate::models::users::User;
use crate::schema::users;
use crate::utils::helpers;
use diesel::prelude::{ExpressionMethods, QueryDsl, RunQueryDsl};
use log::info;

use juniper::FieldError;

pub fn get_user(context: &Context, username: String) -> Result<User, diesel::result::Error> {
    let connection = &mut context.db.establish_connection();
    let user = users::users::table
        .filter(users::users::username.eq(&username))
        .first::<User>(connection);
    // .expect("Error loading user");
    return user;
}

pub fn user_login(
    context: &Context,
    username: String,
    password: String,
) -> Result<User, FieldError> {
    let user_data = get_user(context, username);
    info!("User found: {:?}", user_data);
    match user_data {
        Ok(user_data) => {
            info!("Yes");
            info!("User hash is: {:?}", user_data.hash);
            let is_valid_password = helpers::verify_password(&password, &user_data.hash);
            info!("User is_valid_password: {:?}", is_valid_password);
            if is_valid_password.unwrap() {
                info!("User is valid");
                return Ok(user_data);
            } else {
                info!("User is invalid");
                return Err(FieldError::new("Invalid password", juniper::Value::null()));
            }
        }
        Err(diesel::result::Error::NotFound) => {
            info!("No");
            Err(FieldError::new("No user found", juniper::Value::null()))
        }
        Err(e) => {
            info!("No");
            Err(FieldError::new(
                format!("Error: {:?}", e),
                juniper::Value::null(),
            ))
        }
    }
}

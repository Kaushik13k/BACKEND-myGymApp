use crate::database::connection::Context;
use crate::models::users::User;
use crate::schema::users;
use crate::utils::helpers;
use diesel::prelude::{ExpressionMethods, QueryDsl, RunQueryDsl};
use juniper::FieldError;
use log::info;

pub fn get_user(context: &Context, username: String) -> Result<User, diesel::result::Error> {
    let connection = &mut context.db.establish_connection();
    let user = users::users::table
        .filter(users::users::username.eq(&username))
        .first::<User>(connection);
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
            let is_valid_password = helpers::verify_password(&password, &user_data.hash);
            info!("User is_valid_password: {:?}", is_valid_password);
            if is_valid_password.unwrap() {
                info!("User is valid");
                Ok(user_data)
            } else {
                Err(FieldError::new(
                    "Invalid Credentials",
                    juniper::Value::null(),
                ))
            }
        }
        Err(diesel::result::Error::NotFound) => {
            info!("No user found");
            Err(FieldError::new("No user found", juniper::Value::null()))
        }
        Err(e) => {
            info!("There was an error in the login process: {:?}", e);
            Err(FieldError::new(
                format!("Error: {:?}", e),
                juniper::Value::null(),
            ))
        }
    }
}

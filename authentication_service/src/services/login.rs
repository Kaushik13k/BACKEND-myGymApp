use crate::database::connection::Context;
use crate::models::users::User;
use crate::schema::users;
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

pub fn user_login(context: &Context, username: String) -> Result<User, FieldError> {
    let user_data = get_user(context, username);
    info!("User found: {:?}", user_data);
    match user_data {
        Ok(user_data) => {
            info!("Yes");
            Ok(user_data)
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

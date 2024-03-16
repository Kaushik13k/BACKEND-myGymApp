use crate::database::connection::Context;
use crate::models::users::User;
use crate::schema::users;
use diesel::prelude::{ExpressionMethods, QueryDsl, RunQueryDsl};
use juniper::FieldError;
use log::{error, info};

pub fn get_user(context: &Context, username: String) -> Result<User, diesel::result::Error> {
    info!("Getting user: {:?}", username);
    let connection = &mut context.db.establish_connection();
    let user = users::users::table
        .filter(users::users::username.eq(&username))
        .first::<User>(connection);
    return user;
}

pub fn get_availablity(context: &Context, username: String) -> Result<bool, FieldError> {
    info!("Checking if the user is available: {:?}", username);
    let user_data = get_user(context, username);
    match user_data {
        Ok(user_data) => {
            info!("The query resulted with some data.{:?}", user_data.username);
            Ok(true)
        }
        Err(diesel::result::Error::NotFound) => {
            error!("No user found");
            Ok(false)
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

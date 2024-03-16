use crate::{
    database::connection::Context,
    models::{signup::Signup, user_inputs::InputSignup, users::User},
    schema::users,
    utils::helpers::hash_password,
};
use diesel::prelude::*;
use juniper::FieldError;
use log::{error, info};

pub fn insert_user(context: &Context, new_user: Signup) -> Result<User, diesel::result::Error> {
    let connection = &mut context.db.establish_connection();
    let user = diesel::insert_into(users::users::table)
        .values(&new_user)
        .get_result(connection);
    return user;
}

pub fn user_signup(context: &Context, user_input: InputSignup) -> Result<User, FieldError> {
    let password_hashed = hash_password(&user_input.password);
    let new_user = Signup {
        firstname: &user_input.firstname,
        lastname: &user_input.lastname,
        username: &user_input.username,
        email: &user_input.email,
        hash: &password_hashed.unwrap(),
        phone_number: &user_input.phone_number,
        age: &user_input.age,
    };
    match insert_user(context, new_user) {
        Ok(user) => {
            info!("The user was inserted successfully.");
            Ok(user)
        }
        Err(_) => {
            error!("Failed to insert user");

            Err(FieldError::new(
                "Failed to insert user",
                juniper::Value::null(),
            ))
        }
    }
}
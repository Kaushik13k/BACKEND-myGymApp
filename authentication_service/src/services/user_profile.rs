use crate::models::profile::{InputProfile, Profile};
use crate::{database::connection::Context, models::users::User, schema::users};

use diesel::prelude::*;
use juniper::FieldError;
use log::{error, info};

use super::get_user;

// pub fn insert_user(context: &Context, new_user: Profile) -> Result<User, diesel::result::Error> {
//     info!("Inserting user profile details.");
//     let connection = &mut context.db.establish_connection();
//     let user = diesel::insert_into(users::users::table)
//         .values(&new_user)
//         .get_result(connection);
//     return user;
// }

pub fn update_user(context: &Context, new_user: Profile) -> Result<User, diesel::result::Error> {
    info!("Inserting user");
    let connection = &mut context.db.establish_connection();
    let filter = users::users::table.filter(users::users::username.eq(new_user.username));

    let updated_user = diesel::update(filter)
        .set((
            users::users::firstname.eq(new_user.firstname),
            users::users::lastname.eq(new_user.lastname),
            users::users::phone_number.eq(new_user.phone_number),
            users::users::dob.eq(new_user.dob),
        ))
        .get_result(connection);
    return updated_user;
}

pub fn user_profile(context: &Context, user_input: InputProfile) -> Result<User, FieldError> {
    info!("update user: {:?}", user_input.username);
    let username = user_input.username.clone();
    let user_data = get_user::get_user(context, username);

    match user_data {
        Ok(_) => {
            info!("User  exists");
            let new_user = Profile {
                firstname: user_input.firstname,
                lastname: user_input.lastname,
                username: user_input.username,
                email: user_input.email,
                phone_number: user_input.phone_number,
                dob: user_input.dob,
            };

            info!("Inserting user profile details for user: {:?}", &new_user);
            match update_user(context, new_user) {
                Ok(user) => {
                    info!("The user profile updated successfully.");
                    Ok(user)
                }
                Err(_) => {
                    error!("Failed to update profile");
                    Err(FieldError::new(
                        "Failed to update profile",
                        juniper::Value::null(),
                    ))
                }
            }
        }
        Err(diesel::result::Error::NotFound) => {
            error!("No user found");
            return Err(FieldError::new("No user found", juniper::Value::null()));
        }
        Err(e) => {
            error!("There was an error in the updating the user: {:?}", e);
            return Err(FieldError::new(
                format!("Error: {:?}", e),
                juniper::Value::null(),
            ));
        }
    }
}

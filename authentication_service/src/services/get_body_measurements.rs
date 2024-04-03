use crate::models::body_measurements::UserBodyMeasurements;
// use crate::models::body_measurements::GetBodyMeasurements;
use crate::models::users::User;
use crate::schema::body_measurements;
use crate::schema::users;
use crate::{database::connection::Context, models::body_measurements::BodyMeasurementsResult};

use diesel::prelude::{ExpressionMethods, QueryDsl, RunQueryDsl};
use juniper::FieldError;
use log::{error, info};

pub fn get_user_and_measurements(
    context: &Context,
    username: String,
) -> Result<(User, BodyMeasurementsResult), diesel::result::Error> {
    info!("Getting user and body measurements: {:?}", username);
    let connection = &mut context.db.establish_connection();
    let user = users::users::table
        .filter(users::users::username.eq(&username))
        .first::<User>(connection)?;

    let body_measurement = body_measurements::body_measurements::table
        .filter(body_measurements::body_measurements::user_id.eq(&user.id)) // Assuming there's a user_id field in body_measurements
        .first::<BodyMeasurementsResult>(connection)?;

    return Ok((user, body_measurement));
}

pub fn body_measurements(
    context: &Context,
    username: String,
) -> Result<UserBodyMeasurements, FieldError> {
    info!("Checking if the user is available: {:?}", username);
    let user_data = get_user_and_measurements(context, username);
    match user_data {
        Ok((user, body_measurements)) => {
            let user_clone = user.clone();
            let body_measurements_clone = body_measurements.clone();
            info!(
                "The query resulted with some data.{:?}",
                (user_clone, body_measurements_clone)
            );

            Ok(UserBodyMeasurements {
                user,
                body_measurements,
            })
        }
        Err(diesel::result::Error::NotFound) => {
            error!("No user/measurements found");
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

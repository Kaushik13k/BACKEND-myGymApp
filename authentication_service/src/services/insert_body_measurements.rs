use crate::models::body_measurements::{BodyMeasurementsInput, BodyMeasurementsResult};
use crate::schema::body_measurements;
use crate::{database::connection::Context, models::body_measurements::BodyMeasurements};
use diesel::prelude::*;
use juniper::FieldError;
use log::{error, info};

use super::get_user;

pub fn insert_measurements(
    context: &Context,
    new_user: BodyMeasurements,
) -> Result<BodyMeasurementsResult, diesel::result::Error> {
    info!("Inserting measurements");
    let connection = &mut context.db.establish_connection();
    let body_measurements = diesel::insert_into(body_measurements::body_measurements::table)
        .values(&new_user)
        .get_result(connection);
    return body_measurements;
}

pub fn insert_body_measurements(
    context: &Context,
    user_input: BodyMeasurementsInput,
) -> Result<BodyMeasurementsResult, FieldError> {
    info!("Insert Body-Measurements: {:?}", user_input.username);
    let username = user_input.username.clone();
    let user_data = get_user::get_user(context, username);

    match user_data {
        Ok(user) => {
            info!("User exists");
            info!("id is: {:?}", user.id);
            let new_user = BodyMeasurements {
                user_id: user.id,
                timestamp: user_input.timestamp,
                weight: user_input.weight,
                height: user_input.height,
                weist: user_input.weist,
                neck: user_input.neck,
                shoulders: user_input.shoulders,
                chest: user_input.chest,
                left_bicep: user_input.left_bicep,
                right_bicep: user_input.right_bicep,
                left_forearm: user_input.left_forearm,
                right_forearm: user_input.right_forearm,
                abdomen: user_input.abdomen,
                hips: user_input.hips,
                left_thigh: user_input.left_thigh,
                right_thigh: user_input.right_thigh,
                left_calf: user_input.left_calf,
                right_calf: user_input.right_calf,
            };
            info!("Inserting measurements {:?}", &new_user);
            match insert_measurements(context, new_user) {
                Ok(user) => {
                    info!("The measurements was inserted successfully.");
                    Ok(user)
                }
                Err(e) => {
                    error!("Failed to insert measurements with error: {:?}", e);
                    Err(FieldError::new(
                        "Failed to insert measurements",
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
            error!(
                "There was an error in the while inserting measurements data: {:?}",
                e
            );
            return Err(FieldError::new(
                format!("Error: {:?}", e),
                juniper::Value::null(),
            ));
        }
    }
}

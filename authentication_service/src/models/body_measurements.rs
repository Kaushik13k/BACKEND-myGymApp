use crate::database::connection::Context;
use crate::models::users::User;
use crate::schema::body_measurements;
use diesel::prelude::Queryable;
use diesel::Insertable;

#[derive(Insertable, Debug)]
#[diesel(table_name = body_measurements::body_measurements)]
pub struct BodyMeasurements {
    pub user_id: i32,
    pub weight: Option<f64>,
    pub height: Option<f64>,
    pub weist: Option<f64>,
    pub neck: Option<f64>,
    pub shoulders: Option<f64>,
    pub chest: Option<f64>,
    pub left_bicep: Option<f64>,
    pub right_bicep: Option<f64>,
    pub left_forearm: Option<f64>,
    pub right_forearm: Option<f64>,
    pub abdomen: Option<f64>,
    pub hips: Option<f64>,
    pub left_thigh: Option<f64>,
    pub right_thigh: Option<f64>,
    pub left_calf: Option<f64>,
    pub right_calf: Option<f64>,
    pub timestamp: Option<i32>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct BodyMeasurementsInput {
    pub username: String,
    pub weight: Option<f64>,
    pub height: Option<f64>,
    pub weist: Option<f64>,
    pub neck: Option<f64>,
    pub shoulders: Option<f64>,
    pub chest: Option<f64>,
    pub left_bicep: Option<f64>,
    pub right_bicep: Option<f64>,
    pub left_forearm: Option<f64>,
    pub right_forearm: Option<f64>,
    pub abdomen: Option<f64>,
    pub hips: Option<f64>,
    pub left_thigh: Option<f64>,
    pub right_thigh: Option<f64>,
    pub left_calf: Option<f64>,
    pub right_calf: Option<f64>,
    pub timestamp: Option<i32>,
}

#[derive(Queryable, Debug, juniper::GraphQLObject, Clone)]

pub struct BodyMeasurementsResult {
    pub id: i32,
    pub user_id: i32,
    pub weight: Option<f64>,
    pub height: Option<f64>,
    pub weist: Option<f64>,
    pub neck: Option<f64>,
    pub shoulders: Option<f64>,
    pub chest: Option<f64>,
    pub left_bicep: Option<f64>,
    pub right_bicep: Option<f64>,
    pub left_forearm: Option<f64>,
    pub right_forearm: Option<f64>,
    pub abdomen: Option<f64>,
    pub hips: Option<f64>,
    pub left_thigh: Option<f64>,
    pub right_thigh: Option<f64>,
    pub left_calf: Option<f64>,
    pub right_calf: Option<f64>,
    pub timestamp: Option<i32>,
}

pub struct UserBodyMeasurements {
    pub user: User,
    pub body_measurements: BodyMeasurementsResult,
}

#[juniper::object(Context = Context)]
impl UserBodyMeasurements {
    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn body_measurements(&self) -> &BodyMeasurementsResult {
        &self.body_measurements
    }
}

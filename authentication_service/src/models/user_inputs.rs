#[derive(juniper::GraphQLInputObject)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct UserAvailable {
    pub username: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct InputSignup {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone_number: i32,
    pub age: i32,
}

#[derive(juniper::GraphQLInputObject)]
pub struct ForgotPassword {
    pub username: String,
    pub password: String,
}

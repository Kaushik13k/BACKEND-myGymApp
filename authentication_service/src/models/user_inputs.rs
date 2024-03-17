

#[derive(juniper::GraphQLInputObject)]
pub struct UserAvailable {
    pub username: String,
}



#[derive(juniper::GraphQLInputObject)]
pub struct ForgotPassword {
    pub username: String,
    pub password: String,
}

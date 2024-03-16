use crate::database::connection::Context;
use crate::models::users::User;
use crate::schema::users;
use diesel::prelude::*;
use diesel::Insertable;

#[derive(Insertable)]
#[diesel(table_name = users::users)]

pub struct SignupInput<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub username: &'a str,
    pub email: &'a str,
    pub hash: &'a str,
    pub phone_number: &'a i32,
    pub age: &'a i32,
}

#[derive(juniper::GraphQLInputObject)]
pub struct SignupInputGraphql {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone_number: i32,
    pub age: i32,
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    pub fn signup(context: &Context, user_input: SignupInputGraphql) -> User {
        let connection = &mut context.db.establish_connection();

        let new_user = SignupInput {
            firstname: &user_input.firstname,
            lastname: &user_input.lastname,
            username: &user_input.username,
            email: &user_input.email,
            hash: &user_input.password,
            phone_number: &user_input.phone_number,
            age: &user_input.age,
        };

        diesel::insert_into(users::users::table)
            .values(&new_user)
            .get_result(connection)
            .expect("Error saving new muscle group")
    }
}

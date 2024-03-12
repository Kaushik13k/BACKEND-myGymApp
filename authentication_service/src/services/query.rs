use crate::database::connection::Context;
use crate::models::users::User;
use crate::schema::users;
use diesel::prelude::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub struct QueryRoot;
#[juniper::object(Context = Context)]
impl QueryRoot {
    fn get_user(&self, context: &Context, username: String, email: String) -> User {
        let connection = &mut context.db.establish_connection();
        let user = users::users::table
            .filter(users::users::username.eq(&username))
            .filter(users::users::email.eq(&email))
            .first::<User>(connection)
            .expect("Error loading user");

        user
    }
}

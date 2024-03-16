use crate::database::connection::Context;
use crate::models::users::User;
use crate::schema::users;
use diesel::prelude::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub fn get_user(context: &Context, username: String) -> Result<User, diesel::result::Error> {
    let connection = &mut context.db.establish_connection();
    let user = users::users::table
        .filter(users::users::username.eq(&username))
        .first::<User>(connection);
    return user;
}

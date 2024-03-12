use diesel::prelude::Queryable;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[juniper::object(description = "A user of the system")]
impl User {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn username(&self) -> &str {
        self.username.as_str()
    }

    pub fn email(&self) -> &str {
        self.email.as_str()
    }
}

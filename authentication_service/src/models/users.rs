use diesel::prelude::Queryable;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub hash: String,
    pub phone_number: i32,
    pub age: i32,
}

#[juniper::object(description = "A user of the system")]
impl User {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn firstname(&self) -> &str {
        self.firstname.as_str()
    }

    pub fn lastname(&self) -> &str {
        self.lastname.as_str()
    }

    pub fn username(&self) -> &str {
        self.username.as_str()
    }

    pub fn email(&self) -> &str {
        self.email.as_str()
    }

    pub fn hash(&self) -> &str {
        self.hash.as_str()
    }

    pub fn phone_number(&self) -> i32 {
        self.phone_number
    }

    pub fn age(&self) -> i32 {
        self.age
    }
}

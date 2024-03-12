use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

impl juniper::Context for Context {}
pub struct Context {
    pub db: Db,
}

impl Context {
    pub fn new() -> Self {
        Self { db: Db::new() }
    }
}
pub struct Db;

impl Db {
    pub fn new() -> Self {
        Self
    }

    pub fn establish_connection(&self) -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        print!("Connecting to {}", database_url);
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

pub fn init() -> SqliteConnection {
    let db_url: &str = &env::var("DATABASE_URL")
        .expect("env: DATABASE_URL must be set");
    
    SqliteConnection::establish(db_url)
        .expect(&format!("Error connecting to {}", db_url))
}

infer_schema!("dotenv:DATABASE_URL");

#[derive(Queryable)]
pub struct Issue {
    pub id: i32,
    pub summary: String,
    pub description: String,
    pub priority: i32
}
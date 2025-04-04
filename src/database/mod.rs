use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;
pub mod models;
#[allow(dead_code)]
pub fn connect_database() -> SqliteConnection{
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return SqliteConnection::establish(database_url.as_str()).expect("Error connecting to database");
}

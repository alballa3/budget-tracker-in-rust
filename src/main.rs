use auth::{create_session, Session};
use database::connect_database;

mod auth;
mod budget;
mod database;
mod schema;
fn main() {
    #[allow(unused_variables)]
    let conn = &mut connect_database();
    create_session(Session{
        username: "admin".to_string(),
        password: "admin".to_string(),
    });
    
}

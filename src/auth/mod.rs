use argon2::password_hash::SaltString;

pub struct Session {
    pub username: String,
    pub password: String,
}
// pub fn register(username: &str, password: &str)  {

// }




pub fn create_session(data: Session) {
    let password=SaltString::generate(data.password).to_string();


    let  session = Session {
        username: data.username,
        password: password,
    };
    println!("{:?}", session.password);
}

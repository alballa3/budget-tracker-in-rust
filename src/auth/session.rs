use std::{
    env,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use dotenvy::dotenv;
use jsonwebtoken::Header;
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Session {
    pub username: String,
    pub token: Option<String>,
}
// For The tokens
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Claims {
    sub: String, // subject (e.g., user ID)
    exp: usize,  // expiry timestamp
}
pub fn generate_token(username: &str) -> String {
    dotenv().ok();
    // To Generate The token
    let clamis = Claims {
        sub: username.to_string(),
        exp: 1000000000,
    };
    let key = env::var("JWT_SECRET").expect("failed to secert token");
    let token: String = jsonwebtoken::encode(
        &Header::default(),
        &clamis,
        &jsonwebtoken::EncodingKey::from_secret(key.as_bytes()),
    )
    .expect("Failed to create token");
    return token;
}
pub fn create_session(username:String, token:String) {
    let session = Session {
        username,
        token: Some(token),
    };
    // check if the file exists
    let path = Path::new("session.json");
    if path.exists() {
        panic!("Session file already exists");
    }
    let mut config = File::create("session.json").expect("Failed to create session file");
    let json = serde_json::to_string_pretty(&session).expect("Failed to convert session to string");
    config
        .write_all(json.as_bytes())
        .expect("Failed to write session to file");
}
pub fn get_session() -> Session {
    let mut config = match File::open("session.json") {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                panic!("Please Register First");
            } else {
                panic!("Failed to open session file: {}", e);
            }
        }
    };
    let mut json = String::new();
    config
        .read_to_string(&mut json)
        .expect("Failed to read session file");
    let session: Session =
        serde_json::from_str(&json).expect("Failed to convert session to string");
    session
}

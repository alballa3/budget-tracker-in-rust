use std::fs;

use crate::{
    database::{
        connect_database,
        models::{NewUser, User},
    },
    schema::{self},
};
use diesel::result::{DatabaseErrorKind, Error};

use bcrypt::{DEFAULT_COST, hash};
use dialoguer::{Input, Password};
use diesel::{
    ExpressionMethods, RunQueryDsl, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use session::{create_session, generate_token};
pub mod session;
#[allow(dead_code)]
pub fn register() {
    let name: String = Input::new()
        .with_prompt("Enter your name")
        .interact()
        .expect("Failed to get user input");

    let password = Password::new()
        .with_prompt("Enter your password")
        .with_confirmation("Confirm password", "Passwords mismatching")
        .interact()
        .unwrap();
    let token = generate_token(&name);
    let hash = hash(password, DEFAULT_COST).unwrap();
    let user = NewUser {
        name: &name,
        password: &hash,
        session: &token,
    };

    create_session(name.clone(), token.clone());

    match diesel::insert_into(schema::users::table)
        .values(&user)
        .execute(&mut crate::database::connect_database())
    {
        Ok(_) => {
            println!("User registered successfully");
        }
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            println!("A user with that name already exists.");
        }
        Err(e) => {
            println!("Failed to insert user: {:?}", e);
        }
    }
}
#[allow(dead_code)]
pub fn login() {
    loop {
        let _name: String = Input::new()
            .with_prompt("Enter your name")
            .interact()
            .expect("Failed to get user input");

        let _password = Password::new()
            .with_prompt("Enter your password")
            .interact()
            .unwrap();
        use schema::users::dsl::*;

        let user: Vec<User> = users
            .filter(name.eq(&_name))
            .select(User::as_select())
            .load(&mut connect_database())
            .expect("Error loading user");
        if user.is_empty() {
            println!("User not found");
            continue;
        }
        let user: &User = user.first().unwrap();
        // let data=bcrypt::verify(_password, &user.password).expect("msg");
        match bcrypt::verify(_password, &user.password) {
            Ok(true) => {
                println!("User logged in successfully");
                create_session(user.name.clone(), user.session.to_string());
                break;
            }
            Ok(false) => {
                println!("Invalid password");
                continue;
            }
            Err(e) => {
                eprintln!("Password verification error: {}", e);
                println!("Error verifying password");
                continue;
            }
        }
    }
}
#[allow(dead_code)]

pub fn logout() {
    fs::remove_file("session.json").expect("Failed to remove session file");
    println!("Logged out successfully");
}

#[allow(dead_code)]
pub fn get_user_id() -> Option<i32> {
    let user = session::get_session();

    match schema::users::dsl::users
        .filter(schema::users::dsl::name.eq(&user.username))
        .select(schema::users::dsl::id)
        .first::<Option<i32>>(&mut connect_database())
    {
        Ok(Some(id)) => Some(id),
        Ok(None) => {
            println!("User ID not found for username: {}", user.username);
            None
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            None
        }
    }
}

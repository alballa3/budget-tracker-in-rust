use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
pub mod transaction;
use crate::{
    auth::{get_user_id, start},
    database::{
        connect_database,
        models::{Budget, NewBudget, UpdatedBudget},
    },
    schema::budget::{self, user_id},
};

#[allow(dead_code)]
pub fn add_buget(budget_amount: i32) {
    start();
    let get_user = get_user_id();
    let id = match get_user {
        Some(id) => id,
        None => {
            println!("User not logged in");
            return;
        }
    };
    let mut connect_database = connect_database();
    let check_user: Result<Budget, diesel::result::Error> = budget::table
        .filter(user_id.eq(id))
        .first::<Budget>(&mut connect_database);
    if let Ok(_) = check_user {
        println!("User already has a budget");
        return;
    }
    let budget = NewBudget {
        amount: budget_amount,
        user_id: id,
    };
    diesel::insert_into(budget::table)
        .values(budget)
        .execute(&mut connect_database)
        .expect("Error adding budget");
    println!("Budget added successfully.");
}
#[allow(dead_code)]

pub fn get_budget() -> Option<Budget> {
    start();
    let get_user = get_user_id();
    let id = match get_user {
        Some(id) => id,
        None => {
            println!("User not logged in");
            return None;
        }
    };
    let mut connect_database = connect_database();
    let budget: Result<Budget, diesel::result::Error> = budget::table
        .filter(user_id.eq(id))
        .first::<Budget>(&mut connect_database);
    match budget {
        Ok(budget) => Some(budget),
        Err(_) => {
            return None;
        }
    }
}
#[allow(dead_code)]
pub fn update_budget(budget_amount: i32) {
    start();
    let get_user = get_user_id();
    let id = match get_user {
        Some(id) => id,
        None => {
            println!("User not logged in");
            return;
        }
    };
    let mut connect_database = connect_database();
    let changes = UpdatedBudget {
        amount: budget_amount,
    };
    match diesel::update(budget::table.filter(user_id.eq(id)))
        .set(changes)
        .execute(&mut connect_database)
    {
        Ok(_) => println!("budget Updated"),
        Err(err) => println!("Failed to update: {}", err),
    };
}
#[derive(clap::Subcommand)]
pub enum BudgetCli {
    /// Add a new budget
    Add {
        #[arg(short, long)]
        amount: i32,
    },
    /// Update a budget
    Update {
        #[arg(short, long)]
        amount: i32,
    },
    /// View a budget
    View,
}

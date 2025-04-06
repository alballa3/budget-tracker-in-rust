use std::cmp::Ordering;

use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
use prettytable::{Table, row};

use crate::{
    auth::start,
    database::{
        connect_database,
        models::{NewTransaction, Transaction, UpdatedTransaction},
    },
    schema::{self, transactions},
};

use super::get_budget;
#[allow(dead_code)]

pub fn add_transaction(transaction_amount: i32, description: String) {
    start();
    let get_budget = get_budget();
    let budget = match get_budget {
        Some(budget) => budget,
        None => {
            println!("User does not have a budget");
            return;
        }
    };
    let tranasction: NewTransaction = NewTransaction {
        amount: transaction_amount,
        description,
        budget_id: budget.id,
    };
    diesel::insert_into(schema::transactions::table)
        .values(tranasction)
        .execute(&mut connect_database())
        .expect("Error adding transaction");
    println!("Transaction added successfully.");
}
#[allow(dead_code)]
pub fn list_transactions() {
    start();
    let get_budget = get_budget();
    let budget = match get_budget {
        Some(budget) => budget,
        None => {
            println!("User does not have a budget");
            return;
        }
    };
    let data = transactions::table
        .filter(transactions::budget_id.eq(budget.id))
        .load::<Transaction>(&mut connect_database())
        .expect("Error getting transactions");
    let mut table = Table::new();
    let mut total = 0;
    table.add_row(row!["ID", "Amount", "Description", "created_at"]);
    for transaction in data {
        total += transaction.amount;
        table.add_row(row![
            transaction.id,
            format!("${}", transaction.amount),
            transaction.description,
            format!("{:?}", transaction.created_at.unwrap())
        ]);
    }
    let remaining = match total.cmp(&budget.amount) {
        Ordering::Less => budget.amount - total,
        Ordering::Equal => 0,
        Ordering::Greater => total - budget.amount,
    };

    let remaining_label = if total > budget.amount {
        "Over Budget"
    } else {
        "Remaining"
    };

    println!("Total: ${}", total);
    println!("{}: ${}", remaining_label, remaining);
    table.printstd();
}

#[allow(dead_code)]
pub fn update_transaction(transaction_id: i32, updated_amount: i32) {
    start();
    let changes = UpdatedTransaction {
        amount: updated_amount,
    };
    diesel::update(schema::transactions::table)
        .filter(transactions::id.eq(transaction_id))
        .set(changes)
        .execute(&mut connect_database())
        .expect("Error updating budget");
    println!("Budget updated successfully.");
}
#[allow(dead_code)]
pub fn delete_transaction(transaction_id: i32) {
    start();
    diesel::delete(schema::transactions::table)
        .filter(transactions::id.eq(transaction_id))
        .execute(&mut connect_database())
        .expect("Error deleting transaction");
    println!("Transaction deleted successfully.");
}
#[derive(clap::Subcommand)]
pub enum TransactionCli {
    /// Add a new transaction
    List,
    /// Update a transaction
    Add {
        #[arg(short, long)]
        amount: i32,
        #[arg(short, long)]
        description: String,
    },
    /// Delete a transaction
    Delete {
        #[arg(short, long)]
        id: i32,
    },
    /// Update a transaction
    Update {
        #[arg(short, long)]
        id: i32,
        #[arg(short, long)]
        amount: i32,
    },
}

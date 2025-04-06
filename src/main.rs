use auth::AuthCli;
use budget::{
    BudgetCli,
    transaction::{self, TransactionCli},
};
use clap::Parser;

mod auth;
mod budget;
mod database;
mod schema;

#[allow(dead_code)]
#[derive(clap::Parser)]
#[command(author, version, about, long_about = None,name = "Budget CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(clap::Subcommand)]
enum Commands {
    #[command(subcommand)]
    /// Register a new user Or Login a user Or Logout a user
    Auth(AuthCli),
    /// Add a new budget Or Update a budget Or View a budget
    #[command(subcommand)]
    Budget(BudgetCli),
    #[command(subcommand)]
    /// Add a new transaction Or View all transactions
    Transaction(TransactionCli),
}
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Auth(auth_cli) => match auth_cli {
            AuthCli::Register => auth::register(),
            AuthCli::Login => auth::login(),
            AuthCli::Logout => auth::logout(),
        },
        Commands::Budget(budget_cli) => match budget_cli {
            BudgetCli::Add { amount } => budget::add_buget(amount),
            BudgetCli::View => {
                let budget = budget::get_budget();
                match budget {
                    Some(budget) => println!("Budget: {} $", budget.amount),
                    None => println!("User does not have a budget"),
                }
            }
            BudgetCli::Update { amount } => budget::update_budget(amount),
        },
        Commands::Transaction(transaction_cli) => match transaction_cli {
            TransactionCli::Add {
                amount,
                description,
            } => transaction::add_transaction(amount, description),
            TransactionCli::List => transaction::list_transactions(),
            TransactionCli::Delete { id } => transaction::delete_transaction(id),
            TransactionCli::Update { id, amount } => transaction::update_transaction(id, amount),
        },
    }
}

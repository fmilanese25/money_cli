use clap::{Parser, Subcommand, CommandFactory, Args};
use clap_complete::{generate, shells::Zsh};
use colored::*;
use tabled::{Table, Tabled};
use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(AddArgs),
    List,
    Completions,
}

#[derive(Args)]
struct AddArgs {
    amount: f64,
    category: String,
    #[arg(long)]
    message: Option<String>,
    #[arg(long)]
    date: Option<String>,
    #[arg(long)]
    latitude: Option<f64>,
    #[arg(long)]
    longitude: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Tabled)]
struct Expense {
    id: u32,
    date: String,
    amount: f64,
    category: String,
    #[tabled(display_with = "option_to_string")]
    message: Option<String>,
    #[tabled(display_with = "option_f64_to_string")]
    latitude: Option<f64>,
    #[tabled(display_with = "option_f64_to_string")]
    longitude: Option<f64>,
}

fn option_to_string(opt: &Option<String>) -> String {
    opt.clone().unwrap_or_default()
}

fn option_f64_to_string(opt: &Option<f64>) -> String {
    opt.map(|v| v.to_string()).unwrap_or_default()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add(args) => {
            let expense = serde_json::json!({
                "date": args.date.unwrap_or_else(|| "2025-08-03".to_string()),
                "amount": args.amount,
                "category": args.category,
                "message": args.message,
                "latitude": args.latitude,
                "longitude": args.longitude,
                "image_url": null
            });

            let client = reqwest::Client::new();
            let resp = client.post("http://localhost:8080/expenses")
                .json(&expense)
                .send()
                .await?;

            if resp.status().is_success() {
                println!("{}", "expense created!".green());
            } else {
                println!("{} {:?}", "error creating expense:".red(), resp.text().await?);
            }
        }

        Commands::List => {
            let client = reqwest::Client::new();
            let resp = client.get("http://localhost:8080/expenses")
                .send()
                .await?;

            if resp.status().is_success() {
                let expenses: Vec<Expense> = resp.json().await?;
                let table = Table::new(expenses).to_string();
                println!("{}", table);
            } else {
                println!("{} {:?}", "error fetching expenses:".red(), resp.text().await?);
            }
        }

        Commands::Completions => {
            generate(Zsh, &mut Cli::command(), "money_cli", &mut std::io::stdout());
        }
    }

    Ok(())
}

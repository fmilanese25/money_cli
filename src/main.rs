use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, shells::Zsh};
use colored::*;
use console::strip_ansi_codes;
use reqwest;
use serde::{Deserialize, Serialize};
use tabled::{Style, Table, Tabled};

mod utils;

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
  #[tabled(display_with = "id_color")]
  id: u32,
  #[tabled(display_with = "date_color")]
  date: String,
  #[tabled(display_with = "amount_color")]
  amount: f64,
  #[tabled(display_with = "category_color")]
  category: String,
  #[tabled(display_with = "option_to_colored_string")]
  message: Option<String>,
  #[tabled(display_with = "option_f64_to_colored_string")]
  latitude: Option<f64>,
  #[tabled(display_with = "option_f64_to_colored_string")]
  longitude: Option<f64>,
}

// ===== Display helpers with colors =====
fn id_color(id: &u32) -> String {
  id.to_string().green().to_string()
}

fn date_color(date: &String) -> String {
  date.blue().to_string()
}

fn amount_color(amount: &f64) -> String {
  if *amount > 100.0 {
    format!("{:.2}", amount).red().bold().to_string()
  } else {
    format!("{:.2}", amount).yellow().to_string()
  }
}

fn category_color(category: &String) -> String {
  category.magenta().to_string()
}

fn option_to_colored_string(opt: &Option<String>) -> String {
  opt.clone().unwrap_or_default().cyan().to_string()
}

fn option_f64_to_colored_string(opt: &Option<f64>) -> String {
  opt.map(|v| format!("{:.5}", v).bright_blue().to_string()).unwrap_or_default()
}

fn table_with_colors(expenses: &[Expense]) -> String {
  let mut table = Table::new(expenses).with(Style::blank());
  table = table.with(tabled::Modify::new(tabled::object::Segment::all()).with(|s: &str| strip_ansi_codes(s).to_string()));
  table.to_string()
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
      let resp = client.post("http://localhost:8080/expenses").json(&expense).send().await;

      match resp {
        Ok(r) => {
          if r.status().is_success() {
            epprintln!("info: expense created!");
          } else {
            let text = r.text().await.unwrap_or_default();
            epprintln!("error: creating expense: {}", text);
          }
        }
        Err(_) => {
          epprintln!("error: backend not reachable at http://localhost:8080");
        }
      }
    }

    Commands::List => {
      let client = reqwest::Client::new();
      let resp = client.get("http://localhost:8080/expenses").send().await;

      match resp {
        Ok(r) => {
          if r.status().is_success() {
            let expenses: Vec<Expense> = r.json().await?;
            let table = table_with_colors(&expenses);
            epprintln!("{}", table);
          } else {
            let text = r.text().await.unwrap_or_default();
            epprintln!("error: fetching expenses: {}", text);
          }
        }
        Err(_) => {
          epprintln!("error: backend not reachable at http://localhost:8080");
        }
      }
    }

    Commands::Completions => {
      generate(Zsh, &mut Cli::command(), "money_cli", &mut std::io::stdout());
    }
  }

  Ok(())
}

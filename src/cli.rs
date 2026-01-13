use crate::model::Priority;
use chrono::NaiveDate;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "todo",
    about = "A simple command-line todo app", 
    long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        text: String,

        #[arg(long, value_parser = parse_date)]
        due: Option<NaiveDate>,

        #[arg(long, value_enum, default_value = "medium")]
        priority: Priority,
    },
    List,
    Done {
        id: u32,
    },
    Delete {
        id: u32,
    },
}

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| "Date must be in YYYY-MM-DD format".to_string())
}

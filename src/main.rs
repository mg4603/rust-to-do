use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use chrono::NaiveDate;
use clap::ValueEnum;
use colored::*;

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    ValueEnum,
)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    text: String,
    done: bool,
    due: Option<NaiveDate>,
    #[serde(default = "default_priority")]
    priority: Priority,
}

#[derive(Serialize, Deserialize, Debug)]
struct TaskList {
    next_id: u32,
    tasks: Vec<Task>,
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple command-line todo app", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

fn color_priority(p: Priority, text: &str) -> ColoredString {
    match p {
        Priority::High => text.red(),
        Priority::Medium => text.yellow(),
        Priority::Low => text.blue(),
    }
}

fn color_status(done: bool, text: &str) -> ColoredString {
    if done {
        text.dimmed()
    } else {
        text.normal()
    }
}

fn default_priority() -> Priority {
    Priority::Medium
}

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| "Date must be in YYYY-MM-DD format".to_string())
}

const FILE: &str = "tasks.json";

fn load_tasks() -> TaskList {
    if !Path::new(FILE).exists() {
        return TaskList { 
            next_id: 1,
            tasks: Vec::new(),
        };
    }

    let data = fs::read_to_string(FILE).expect("Failed to read file");
    serde_json::from_str(&data).unwrap_or(TaskList { 
        next_id: 1,
        tasks: Vec::new(),
    })
}

fn save_tasks(list: &TaskList) {
    let data = serde_json::to_string_pretty(list).expect("Failed to serialize");
    fs::write(FILE, data).expect("Failed to write file");
}

fn add_task(text: &str, due: Option<NaiveDate>, priority: Priority) {
    let mut list = load_tasks();
    let id = list.next_id;
    list.next_id += 1;

    list.tasks.push(Task {
        id,
        text: text.to_string(),
        done: false,
        due,
        priority,
    });

    save_tasks(&list);
    println!("Added task #{id}: {text}");
}

fn list_tasks () {
    let mut list = load_tasks();

    if list.tasks.is_empty() {
        println!("No tasks yet.");
        return;
    }

    // Sort: unfinished first, then priority, then due date
    list.tasks.sort_by_key(|t| (
        t.done,
        std::cmp::Reverse(t.priority),
        t.due,
    ));

    for task in list.tasks {
        let status = if task.done { "[x]" } else { "[ ]" };

        let due = match task.due {
            Some(d) => format!(" (due {})", d),
            None => String::new(),
        };
        
        let priority = format!("{:?}", task.priority).to_lowercase();
        let priority_colored = color_priority(task.priority, &priority);

        let text = format!("{}{}", task.text, due);
        let text_colored = color_status(task.done, &text);

        println!(
            "{} {} [{}] {}", 
            status, 
            task.id, 
            priority_colored,
            text_colored
        );
    }
}

fn complete_task(id: u32) {
    let mut list = load_tasks();

    if let Some(task) = list.tasks.iter_mut().find(|t| t.id == id) {
        task.done = true;
        save_tasks(&list);
        println!("Completed task #{id}");
    } else {
        println!("Task #{id} not found");
    }
}

fn delete_task(id: u32) {
    let mut list = load_tasks();
    let before = list.tasks.len();

    list.tasks.retain(|t| t.id != id);

    if list.tasks.len() == before {
        println!("Task #{id} not found");
        return;
    } 

    save_tasks(&list);
    println!("Deleted task #{id}");
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { text, due, priority } => add_task(&text, due, priority),
        Commands::List => list_tasks(),
        Commands::Done { id } => complete_task(id),
        Commands::Delete { id } => delete_task(id),
    }
}


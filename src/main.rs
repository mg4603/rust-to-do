mod cli;
mod model;
mod storage;

use clap::Parser;
use cli::*;
use colored::*;
use model::*;
use storage::*;

fn color_priority(p: Priority, text: &str) -> ColoredString {
    match p {
        Priority::High => text.red(),
        Priority::Medium => text.yellow(),
        Priority::Low => text.blue(),
    }
}

fn color_status(done: bool, text: &str) -> ColoredString {
    if done { text.dimmed() } else { text.normal() }
}

fn list_tasks() {
    let mut list = load_tasks();

    if list.tasks.is_empty() {
        println!("No tasks yet.");
        return;
    }

    // Sort: unfinished first, then priority, then due date
    list.tasks
        .sort_by_key(|t| (t.done(), std::cmp::Reverse(t.priority()), t.due()));

    for task in list.tasks {
        let status = if task.done() { "[x]" } else { "[ ]" };

        let due = match task.due() {
            Some(d) => format!(" (due {})", d),
            None => String::new(),
        };

        let priority = format!("{:?}", task.priority()).to_lowercase();
        let priority_colored = color_priority(task.priority(), &priority);

        let text = format!("{}{}", task.text(), due);
        let text_colored = color_status(task.done(), &text);

        println!(
            "{} {} [{}] {}",
            status,
            task.id(),
            priority_colored,
            text_colored
        );
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            text,
            due,
            priority,
        } => {
            let mut list = load_tasks();
            let id = list.add(text, due, priority);
            save_tasks(&list);
            println!("Added task #{id}");
        }

        Commands::List => list_tasks(),

        Commands::Done { id } => {
            let mut list = load_tasks();
            if list.complete(id) {
                save_tasks(&list);
                println!("Completed task #{id}");
            } else {
                println!("Task #{id} not found");
            }
        }

        Commands::Delete { id } => {
            let mut list = load_tasks();
            if list.delete(id) {
                save_tasks(&list);
                println!("Deleted task #{id}");
            } else {
                println!("Task #{id} not found");
            }
        }
    }
}

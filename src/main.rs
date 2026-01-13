mod cli;
mod model;
mod storage;

use crate::model::TaskList;
use chrono::NaiveDate;
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

fn list_tasks() {
    let mut list = load_tasks();

    if list.tasks.is_empty() {
        println!("No tasks yet.");
        return;
    }

    // Sort: unfinished first, then priority, then due date
    list.tasks
        .sort_by_key(|t| (t.done, std::cmp::Reverse(t.priority), t.due));

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
            status, task.id, priority_colored, text_colored
        );
    }
}

// make toggle?
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_task_increments_id() {
        let mut list = TaskList::new();

        let id1 = list.add("Task 1".to_string(), None, Priority::Medium);
        let id2 = list.add("Task 2".to_string(), None, Priority::Medium);

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(list.tasks.len(), 2);
    }

    #[test]
    fn complete_marks_task_done() {
        let mut list = TaskList::new();
        let id = list.add("Test".to_string(), None, Priority::Medium);

        let ok = list.complete(id);

        assert!(ok);
        assert!(list.tasks[0].done);
    }

    #[test]
    fn complete_unknown_id_fails() {
        let mut list = TaskList::new();

        let ok = list.complete(42);

        assert!(!ok);
    }

    #[test]
    fn delete_removes_task() {
        let mut list = TaskList::new();
        let id1 = list.add("Task 1".to_string(), None, Priority::Medium);
        let id2 = list.add("Task 2".to_string(), None, Priority::Medium);

        let ok = list.delete(id1);

        assert!(ok);
        assert_eq!(list.tasks.len(), 1);
        assert_eq!(list.tasks[0].id, id2);
    }
}

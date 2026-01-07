use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    text: String,
    done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TaskList {
    tasks: Vec<Task>,
}

const FILE: &str = "tasks.json";

fn load_tasks() -> TaskList {
    if !Path::new(FILE).exists() {
        return TaskList { tasks: Vec::new() };
    }

    let data = fs::read_to_string(FILE).expect("Failed to read file");
    serde_json::from_str(&data).unwrap_or(TaskList { tasks: Vec::new() })
}

fn save_tasks(list: &TaskList) {
    let data = serde_json::to_string_pretty(list).expect("Failed to serialize");
    fs::write(FILE, data).expect("Failed to write file");
}

fn add_task(text: &str) {
    let mut list = load_tasks();
    let id = (list.tasks.len() as u32) + 1;

    list.tasks.push(Task {
        id,
        text: text.to_string(),
        done: false,
    });

    save_tasks(&list);
    println!("Added task #{id}: {text}");
}

fn list_tasks () {
    let list = load_tasks();

    if list.tasks.is_empty() {
        println!("No tasks yet.");
        return;
    }

    for task in list.tasks {
        let status = if task.done { "[x]" } else { "[ ]" };
        println!("{} {} {}", status, task.id, task.text);
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
    println!("Hello, world!");
}

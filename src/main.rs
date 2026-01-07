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

fn main() {
    println!("Hello, world!");
}

use std::fs;

use crate::model::TaskList;

const FILE: &str = "tasks.json";

pub fn load_tasks() -> TaskList {
    match fs::read_to_string(FILE) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| TaskList::new()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => TaskList::new(),
        Err(err) => panic!("Failed to read tasks file: {err}"),
    }
}

pub fn save_tasks(list: &TaskList) {
    let data = serde_json::to_string_pretty(list).expect("Failed to serialize");
    fs::write(FILE, data).expect("Failed to write file");
}

use std::fs;
use std::path::Path;

use crate::model::TaskList;

const FILE: &str = "tasks.json";

pub fn load_tasks() -> TaskList {
    if !Path::new(FILE).exists() {
        TaskList::new();
    }

    let data = fs::read_to_string(FILE).expect("Failed to read file");
    serde_json::from_str(&data).unwrap_or_else(|_| TaskList::new())
}

pub fn save_tasks(list: &TaskList) {
    let data = serde_json::to_string_pretty(list).expect("Failed to serialize");
    fs::write(FILE, data).expect("Failed to write file");
}

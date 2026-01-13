use chrono::NaiveDate;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Priority {
    Low,
    Medium,
    High,
}

fn default_priority() -> Priority {
    Priority::Medium
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub text: String,
    pub done: bool,
    pub due: Option<NaiveDate>,
    #[serde(default = "default_priority")]
    pub priority: Priority,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub next_id: u32,
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList {
            next_id: 1,
            tasks: Vec::new(),
        }
    }

    pub fn add(&mut self, text: String, due: Option<NaiveDate>, priority: Priority) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        self.tasks.push(Task {
            id,
            text,
            done: false,
            due,
            priority,
        });
        id
    }

    pub fn complete(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.done = true;
            true
        } else {
            false
        }
    }

    pub fn delete(&mut self, id: u32) -> bool {
        let before = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        before != self.tasks.len()
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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub command: String,
    pub dependencies: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DAG {
    pub tasks: HashMap<String, Task>,
}

impl DAG {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.id.clone(), task);
    }

    pub fn get_dependencies(&self, task_id: &str) -> Option<&Vec<String>> {
        self.tasks.get(task_id).map(|task| &task.dependencies)
    }
}

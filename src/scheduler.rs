use crate::dag::DAG;
use std::collections::HashSet;

pub fn schedule_tasks(dag: &DAG) -> Vec<String> {
    let mut scheduled = Vec::new();
    let mut seen = HashSet::new();

    for task_id in dag.tasks.keys() {
        if !seen.contains(task_id) {
            dfs(task_id, dag, &mut seen, &mut scheduled);
        }
    }

    scheduled
}

fn dfs(task_id: &str, dag: &DAG, seen: &mut HashSet<String>, scheduled: &mut Vec<String>) {
    if let Some(deps) = dag.get_dependencies(task_id) {
        for dep in deps {
            if !seen.contains(dep) {
                dfs(dep, dag, seen, scheduled);
            }
        }
    }
    seen.insert(task_id.to_string());
    scheduled.push(task_id.to_string());
}

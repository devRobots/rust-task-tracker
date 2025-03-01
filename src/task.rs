use chrono::offset::Local;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum Status {
    Done,
    Todo,
    InProgress,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u16,
    pub description: Option<String>,
    pub status: Status,
    pub created_at: String,
    pub updated_at: String,
}

pub fn add_task(tasks: &mut Vec<Task>, description: String) {
    let task = Task {
        id: tasks.len() as u16 + 1,
        description: Some(description.to_string()),
        status: Status::Todo,
        created_at: Local::now().to_rfc3339(),
        updated_at: Local::now().to_rfc3339(),
    };
    tasks.push(task);
}

pub fn update_task(tasks: &mut Vec<Task>, id: u16, description: String) {
    for task in tasks {
        if task.id == id {
            task.description = Some(description.to_string());
            task.updated_at = Local::now().to_rfc3339();
        }
    }
}

pub fn delete_task(tasks: &mut Vec<Task>, id: u16) {
    tasks.retain(|task: &Task| task.id != id);
}

pub fn mark_in_progress(tasks: &mut Vec<Task>, id: u16) {
    for task in tasks {
        if task.id == id {
            task.status = Status::InProgress;
            task.updated_at = Local::now().to_rfc3339();
        }
    }
}

pub fn mark_done(tasks: &mut Vec<Task>, id: u16) {
    for task in tasks {
        if task.id == id {
            task.status = Status::Done;
            task.updated_at = Local::now().to_rfc3339();
        }
    }
}

pub fn list_tasks(tasks: &Vec<Task>, maybe_status: Option<&String>) {
    let status: Option<Status> = match maybe_status {
        Some(status) => match status.as_str() {
            "done" => Some(Status::Done),
            "todo" => Some(Status::Todo),
            _ => Some(Status::InProgress)
        },
        None => None,
    };

    let mut filtered_tasks: Vec<Task> = tasks.clone();
    if let Some(status_) = status {
        filtered_tasks.retain(|task: &Task| task.status == status_);
    }

    for task in filtered_tasks {
        println!("{:?}", task);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut tasks: Vec<super::Task> = Vec::new();
        add_task(&mut tasks, "Task 1".to_string());
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, Some("Task 1".to_string()));    
    }

    #[test]
    fn test_update_task() {
        let mut tasks: Vec<super::Task> = Vec::new();
        add_task(&mut tasks, "Task 1".to_string());
        update_task(&mut tasks, 1, "Task 2".to_string());
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, Some("Task 2".to_string()));
    }

    #[test]
    fn test_delete_task() {
        let mut tasks: Vec<super::Task> = Vec::new();
        add_task(&mut tasks, "Task 1".to_string());
        delete_task(&mut tasks, 1);
        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn test_mark_in_progress() {
        let mut tasks: Vec<super::Task> = Vec::new();
        add_task(&mut tasks, "Task 1".to_string());
        mark_in_progress(&mut tasks, 1);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].status, Status::InProgress);
    }

    #[test]
    fn test_mark_done() {
        let mut tasks: Vec<super::Task> = Vec::new();
        add_task(&mut tasks, "Task 1".to_string());
        mark_done(&mut tasks, 1);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].status, Status::Done);
    }
}
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

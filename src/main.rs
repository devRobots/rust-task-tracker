use std::env;

mod file;
mod task;
use task::{Task, add_task, delete_task, list_tasks, mark_done, mark_in_progress, update_task};

fn main() {
    let mut tasks: Vec<Task> = file::load("tasks.json");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        return;
    }

    let action: &String = &args[1];
    match action.as_str() {
        "add" => {
            if args.len() < 3 {
                usage();
                return;
            }
            let description: String = args[2..].join(" ");
            add_task(&mut tasks, description)
        }
        "update" => {
            if args.len() < 4 {
                usage();
                return;
            }
            let id: u16 = args[2].parse::<u16>().unwrap();
            let description: String = args[3..].join(" ");
            update_task(&mut tasks, id, description)
        }
        "delete" => {
            if args.len() != 3 {
                usage();
                return;
            }
            let id: u16 = args[2].parse::<u16>().unwrap();
            delete_task(&mut tasks, id)
        }
        "mark-in-progress" => {
            if args.len() != 3 {
                usage();
                return;
            }
            let id: u16 = args[2].parse::<u16>().unwrap();
            mark_in_progress(&mut tasks, id)
        }
        "mark-done" => {
            if args.len() != 3 {
                usage();
                return;
            }
            let id: u16 = args[2].parse::<u16>().unwrap();
            mark_done(&mut tasks, id)
        }
        "list" => {
            if args.len() > 3 {
                usage();
                return;
            }
            let filter = args.get(3);
            list_tasks(tasks.clone(), filter)
        }
        _ => usage(),
    }

    file::save(&tasks, "tasks.json");
}

fn usage() {
    println!("Usage: task-cli <action>");
    println!("Actions:");
    println!("  add <task>");
    println!("  update <id> <new-task>");
    println!("  delete <id>");
    println!("  mark-in-progress <id>");
    println!("  mark-done <id>");
    println!("  list [done|todo|in-progress]");
}

use cli_table::Style;
use cli_table::{Cell, Table, format::Justify, };
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::io::{self, Write};
use chrono::prelude::*;

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
enum TaskPriority {
    P0,
    P1,
    P2,
}

impl fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskPriority::P0 => write!(f, "P0"),
            TaskPriority::P1 => write!(f, "P1"),
            TaskPriority::P2 => write!(f, "P2"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    id: String,
    text: String,
    created_at: String,
    updated_at: String,
    priority: TaskPriority,
}

fn add_task(args: &Vec<String>) {
    let mut tasks = get_tasks();

    let next_id = tasks.len() + 1;

    let task = Task {
        id: next_id.to_string(),
        text: args[3].to_string(),
        created_at: Local::now().to_string(),
        updated_at: Local::now().to_string(),
        priority: if args[4].to_lowercase() == "p0" {
            TaskPriority::P0
        } else if args[4].to_lowercase() == "p2" {
            TaskPriority::P2
        } else {
            TaskPriority::P1
        },
    };

    tasks.push(task);

    let updated_json = serde_json::to_string_pretty(&tasks).expect("Failed to update JSON");
    fs::write("src/db/data.json", updated_json).expect("Cannot write");

    println!("Task successfully added!")
}

fn show_task_list() {
    let tasks = get_tasks();
    let task_body: Vec<Vec<_>> = tasks
        .iter()
        .map(|task| {
            vec![
                task.id.as_str().cell(),
                task.text.as_str().cell().justify(Justify::Right),
                task.priority.cell(),
                task.created_at.as_str().cell(),
                task.updated_at.as_str().cell(),
            ]
        })
        .collect();
    let table = task_body
        .table()
        .title(vec![
            "ID".cell().bold(true),
            "Tasks".cell().bold(true),
            "Priority".cell().bold(true),
            "Created At`".cell().bold(true),
            "Updated At`".cell().bold(true),
        ])
        .bold(true);

    let table_display = table.display().unwrap();
    println!("{}", table_display);
}

fn get_tasks() -> Vec<Task> {
    let json_content = fs::read_to_string("src/db/data.json").expect("Cannot read the JSON file");
    let tasks: Vec<Task> =
        serde_json::from_str(&json_content).expect("Cannot parse file data");
    tasks
}

fn main() {
    // read_from_print();
    let args: Vec<String> = std::env::args().collect();

    // for arg in args {
    //     println!("{}", arg);
    // }
    print!("Checking Arg {}", args[2]);

    if args[2] == "add" {
        add_task(&args);
    }
    if args[2] == "list" {
        show_task_list();
    }
}

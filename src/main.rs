use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, Write};

const FILE_PATH: &str = "tasks.json";

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks: HashMap<u32, Task> = load_tasks();

    loop {
        println!("\nTo-Do List Manager");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Complete");
        println!("4. Delete Task");
        println!("5. Quit");

        let choice = get_input("Choose an option: ");

        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => list_tasks(&tasks),
            "3" => mark_complete(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                save_tasks(&tasks);
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 5."),
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn add_task(tasks: &mut HashMap<u32, Task>) {
    let description = get_input("Enter task description: ");
    let id = tasks.len() as u32 + 1;

    tasks.insert(
        id,
        Task {
            description,
            completed: false,
        },
    );

    println!("Task added successfully!");
}

fn list_tasks(tasks: &HashMap<u32, Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
        return;
    }

    println!("\nTasks:");
    for (id, task) in tasks {
        let status = if task.completed { "✔" } else { "✘" };
        println!("{}: {} [{}]", id, task.description, status);
    }
}

fn mark_complete(tasks: &mut HashMap<u32, Task>) {
    let id: u32 = match get_input("Enter task ID to mark as complete: ").parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid ID.");
            return;
        }
    };

    if let Some(task) = tasks.get_mut(&id) {
        task.completed = true;
        println!("Task marked as complete.");
    } else {
        println!("Task not found.");
    }
}

fn delete_task(tasks: &mut HashMap<u32, Task>) {
    let id: u32 = match get_input("Enter task ID to delete: ").parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid ID.");
            return;
        }
    };

    if tasks.remove(&id).is_some() {
        println!("Task deleted successfully.");
    } else {
        println!("Task not found.");
    }
}

fn save_tasks(tasks: &HashMap<u32, Task>) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH);

    match file {
        Ok(mut file) => {
            let data = serde_json::to_string(tasks).expect("Failed to serialize tasks.");
            file.write_all(data.as_bytes())
                .expect("Failed to write to file.");
        }
        Err(_) => println!("Failed to save tasks to file."),
    }
}

fn load_tasks() -> HashMap<u32, Task> {
    let file = OpenOptions::new().read(true).open(FILE_PATH);

    match file {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| HashMap::new())
        }
        Err(_) => HashMap::new(),
    }
}

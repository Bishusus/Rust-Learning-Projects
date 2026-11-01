mod AddTask;
mod ListTask;
mod CompleteTask;
mod DeleteTask;
use std::io::{self, Write};

struct Tasks {
    id: u8,
    name: String,
    description: String,
    completed: bool,
}

fn main() {
    let mut Tasks: Vec<Tasks> = Vec::new();
    loop {
        println!("{}", "=".repeat(50));
        print!("{}", " ".repeat(15));
        println!("Welcome to Task Manager");
        println!("{}", "=".repeat(50));
        println!();
        println!();
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Mark Task as Complete");
        println!("4. Delete Task");
        println!("5. Exit");
        println!();
        print!("Enter your choice(1,2,3,4,5): ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice : u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 5.");
                return;
            }
        };
        if choice < 1 || choice > 5 {
            println!("Invalid input. Please enter a number between 1 and 5.");
            return;
        }
        match choice {
            1 => {
                // Add Task
                let mut id = String::new();
                print!("Enter task id: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id : u8 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid task id.");
                        return;
                    }
                };
                let mut name = String::new();
                print!("Enter task name: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();
                let mut description = String::new();
                print!("Enter task description: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut description).expect("Failed to read line");
                let description = description.trim().to_string();
                let task = Tasks {
                    id,
                    name,
                    description,
                    completed: false,
                };
                AddTask::add_task(&mut Tasks, task);
            }

            2 => {
                // View Tasks
                ListTask::display(&Tasks);
            }

            3 => {
                // Mark Task as Complete
                CompleteTask::complete(&mut Tasks);
            }

            4 => {
                // Delete Task
                DeleteTask::delete(&mut Tasks);
            }

            5 => {
                // Exit
                println!("Exiting...");
                return;
            }
                _ => println!("Invalid choice"),
        };
    }   
}
use std::io::{self, Write};
use crate::Tasks;

pub fn complete(tasks: &mut Vec<Tasks>) {
    if tasks.is_empty() {
        println!("No tasks available.");
        return;
    }
    print!("Enter the ID of the task to mark as complete: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read line");
    let id : u8 = match id.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid task id.");
            return;
        }
    };
    for task in tasks.iter_mut() {
        if task.id == id {
            task.completed = true;
            println!("Task with ID {} marked as complete.", id);
            return;
        }
    }
    println!("Task with ID {} not found.", id);
    return;
}
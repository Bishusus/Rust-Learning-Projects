use crate::Tasks;
use std::io::{self, Write};

pub fn delete(tasks: &mut Vec<Tasks>){
    if tasks.is_empty() {
        println!("No tasks available.");
        return;
    }
    print!("Enter the ID of the task to delete: ");
    let mut id = String::new();
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut id).expect("Failed to read line");
    let id : u8 = match id.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid task id.");
            return;
        }
    };
    match tasks.iter().position(|task| task.id == id) {
        Some(index) => {
            tasks.remove(index);
            println!("Task with ID {} deleted successfully.", id);
        }
        None => {
            println!("Task with ID {} not found.", id);
        }
    }
}
    
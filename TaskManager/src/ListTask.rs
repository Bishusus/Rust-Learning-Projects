use crate::Tasks;

pub fn display(tasks: &Vec<Tasks>) {
    if tasks.is_empty() {
        println!("No tasks available.");
    } else { 
        for task in tasks {
            println!("Task ID: {}", task.id);
            println!("Task Name: {}", task.name);
            println!("Task Description: {}", task.description);
            println!("Task Completed: {}", task.completed);
            println!();
        }
    }
}
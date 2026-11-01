use crate::Tasks;

pub fn add_task(tasks: &mut Vec<Tasks>, task: Tasks) {
    tasks.push(task);
    println!("Task added successfully!");
}
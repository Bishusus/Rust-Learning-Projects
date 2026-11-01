use std::io::{self, Write};
mod files;
mod search;

fn main() {
    println!("Welcome to the Search Engine!");
    let index = files::build_index();
    loop {
        print!("Enter your search query (or type 'exit' to quit): ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut query = String::new();
        io::stdin().read_line(&mut query).expect("Failed to read input");
        let query = query.trim().to_lowercase();
        if query == "exit" {
            println!("Exiting the Search Engine. Goodbye!");
            break;
        }
        if query.is_empty() {
            println!("Search query cannot be empty.");
            continue;
        }
        println!("You searched for: {}", query);
        search::search(&query, &index);
    }
}
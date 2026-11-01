use crate::database::Database;
use std::io::{self, Write};
use crate::error::DatabaseError;

pub fn run_cli(db: &mut Database) {
    // TODO: Implement the CLI
    println!("Available commands:\n");
    println!("1. set <key> <value> - Set a key-value pair\n");
    println!("2. get <key> - Get the value for a key\n");
    println!("3. delete <key> - Delete a key-value pair\n");
    println!("4. update <key> <value> - Update the value for a key\n");
    println!("5. display - Display all key-value pairs\n");
    println!("5. exit - Exit the CLI\n");
    println!();

    loop {
        print!(">");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        let parts : Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let command = parts[0].to_lowercase();
        match command.as_str() {
            "set" => {
                if parts.len() != 3 {
                    println!("Usage: set <key> <value>\n");
                    continue;
                }
                let key = parts[1].to_string();
                let value = parts[2].to_string();
                match db.set(key, value) {
                    Ok(_) => println!("Key-value pair set successfully.\n"),
                    Err(e) => println!("Error: {:?}\n", e),
                }
            }
            "get" => {
                if parts.len() != 2{
                    println!("Usage: get <key>\n");
                    continue;
                }
                let key = parts[1];
                match db.get(key) {
                    Ok(value) => println!("Value: {}\n", value),
                    Err(DatabaseError::KeyNotFound(_)) => println!("Key not found.\n"),
                    Err(e) => println!("Error: {:?}\n", e),
                }
            }
            "delete" => {
                if parts.len() !=2 {
                    println!("Usage: delete <key>\n");
                    continue;
                }
                let key = parts[1];
                match db.delete(key) {
                    Ok(_) => println!("Key-value pair deleted successfully.\n"),
                    Err(DatabaseError::KeyNotFound(_)) => println!("Key not found.\n"),
                    Err(e) => println!("Error: {:?}\n", e),
                }
                
            }
            "update" => {
                if parts.len() != 3 {
                    println!("Usage: update <key> <value>\n");
                    continue;
                }            
                let key = parts[1].to_string();
                let value = parts[2].to_string();
                match db.update(key, value) {
                    Ok(_) => println!("Key-value pair updated successfully.\n"),
                    Err(DatabaseError::KeyNotFound(_)) => println!("Key not found.\n"),
                    Err(e) => println!("Error: {:?}\n", e),
                }
            }
            "display" => {
                db.display();
                println!();
            }
            "exit" => {
                println!("Exiting the CLI. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid command. Please try again.\n");
            }
        }
    }
}

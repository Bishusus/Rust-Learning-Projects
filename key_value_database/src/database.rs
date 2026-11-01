use std::collections::HashMap;
use crate::error::DatabaseError;
use crate::storage::{append_record, read_records};

pub struct Database {
    data: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Result<Self, DatabaseError> {
        
        let mut db = Database {
            data: HashMap::new()
        };

        match read_records() {
            Ok(records) => {
                for record in records {
                    db.reconstruct_from_log(&record);
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // If the log file does not exist, we can start with an empty database
                // Nothing to do since db already initialized as empty
            }
            Err(e) => {
                return Err(DatabaseError::from(e));
            }

        }

        Ok(db)
    }

    pub fn set(&mut self, key: String, value: String) -> Result<bool, DatabaseError> {
        let record = format!("set {} {}", key, value);
        append_record(&record)?;
        self.data.insert(key, value);
        Ok(true)
    }

    pub fn get(&self, key: &str) -> Result<&String, DatabaseError> { 
        match self.data.get(key) {
            Some(value) => Ok(value),
            None => Err(DatabaseError::KeyNotFound(key.to_string())),
        }
    }

    pub fn delete(&mut self, key: &str) -> Result<bool, DatabaseError> {
        if self.data.contains_key(key) {
            let record = format!("delete {}", key);
            append_record(&record)?;
            self.data.remove(key);
            Ok(true)
        } else {
            Err(DatabaseError::KeyNotFound(key.to_string()))
        }
    }

    pub fn update(&mut self, key: String, value: String) -> Result<bool, DatabaseError> {
        if self.data.contains_key(&key) {
            let record = format!("update {} {}", key, value);
            append_record(&record)?;
            self.data.insert(key, value);
            Ok(true)
        }
        else {
            Err(DatabaseError::KeyNotFound(key.to_string()))
        }
    }

    pub fn display(&self) {
        if self.data.is_empty() {
            println!("The database is empty.");
            return;
        }

        for (key, value) in &self.data {
            println!("{}: {}", key, value);
        }
    }

    fn reconstruct_from_log(&mut self, record: &str) {
        let input = record;
        let parts : Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }
        let command = parts[0].to_lowercase();
        match command.as_str() {
            "set" => {
                if parts.len() != 3 {
                    println!("Invalid record in log file: {}", input);
                    return;
                }
                let key = parts[1].to_string();
                let value = parts[2].to_string();
                self.data.insert(key, value);
            }
            "delete" => {
                if parts.len() != 2 {
                    println!("Invalid record in log file: {}", input);
                    return;
                }
                let key = parts[1];
                self.data.remove(key);
            }
            "update" => {       
                if parts.len() != 3 {
                    println!("Invalid record in log file: {}", input);
                    return;
                }   
                let key = parts[1].to_string();
                let value = parts[2].to_string();
                self.data.insert(key, value);
            }
            _ => {
                println!("Invalid command in log file: {}", command);
            }
        }
    }
}
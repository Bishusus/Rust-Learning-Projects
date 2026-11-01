use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn search(query: &str, index: &HashMap<String, HashSet<String>>) -> bool {
    let query = query.trim().to_lowercase();
    let mut found = false;

    if let Some(files) = index.get(&query) {
        for file in files {
            let path = Path::new("data").join(file);
            let contents = match std::fs::read_to_string(&path) {
                Ok(data) => data,
                Err(e) => {
                    println!("Error reading file {}: {}", path.display(), e);
                    continue;
                }
            };
            for (line_index, line) in contents.lines().enumerate() {
                if line.to_lowercase().contains(&query) {
                    println!("Found in {} at line {}: {}", file, line_index + 1, line);
                    found = true;
                }
            }
        }
    }
    else {
        println!("No results found for '{}'.", query);
    }
    found
}
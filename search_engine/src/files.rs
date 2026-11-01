use std::fs;
use std::collections::{HashMap, HashSet};

pub fn build_index() -> HashMap<String, HashSet<String>> {
    let mut index : HashMap<String, HashSet<String>> = HashMap::new();
    // Indexing logic goes here.
    let entries : fs::ReadDir = match fs::read_dir("data") {
        Ok(dir) => dir,
        Err(e) => {
            println!("Error reading directory: {}", e);
            return index;
        }
    };
    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if !path.is_file() || path.extension().is_none_or(|ext| ext != "txt") {
                    continue;
                }
                let filename = path.file_name().unwrap().to_string_lossy().into_owned();
                let contents = match fs::read_to_string(&path) {
                    Ok(data) => data,
                    Err(e) => {
                        println!("Error reading file {}: {}", path.display(), e);
                        continue;
                    }
                };
                for word in contents.split_whitespace() {
                    index
                    .entry(word.to_lowercase())
                    .or_insert_with(HashSet::new)
                    .insert(filename.clone());
                }
            }
            Err(e) => {
                println!("Error reading entry: {}", e);
            }
        }
    }
    index
}
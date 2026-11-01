use std::fs::{self,OpenOptions};
use std::io::Write;

pub fn append_record(record: &str) -> Result<(), std::io::Error> {

    let mut file : std::fs::File = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        ?;
    
    writeln!(file, "{}", record)?;

    Ok(())    
}

pub fn read_records() -> Result<Vec<String>, std::io::Error> {

    let contents : String = fs::read_to_string("log.txt")?;

    let records: Vec<String> = contents.lines().map(String::from).collect();

    Ok(records)
}
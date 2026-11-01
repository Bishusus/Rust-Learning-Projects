use std::io;

#[derive(Debug)]
pub enum DatabaseError {
    KeyNotFound(String),
    InvalidCommand(String),
    InvalidInput(String),
    Io(io::Error),
}

impl From<io::Error> for DatabaseError {
    fn from(error: io::Error) -> Self {
        DatabaseError::Io(error)
    }
}
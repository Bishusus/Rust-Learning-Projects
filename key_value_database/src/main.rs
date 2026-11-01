use key_value_database::database::Database;
use key_value_database::cli::run_cli;


fn main() {
    let mut db = match Database::new() {
        Ok(database) => database,
        Err(e) => {
            eprintln!("Failed to initialize the database: {:?}", e);
            return;
        }
    };

    print!("Welcome to the Key-Value Database CLI!\n");
    run_cli(&mut db);
}

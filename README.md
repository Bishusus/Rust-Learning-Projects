# Rust Simple Projects

A collection of three small Rust command-line projects for practicing file
handling, data structures, indexing, and interactive program design.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) with Cargo

Each project is an independent Cargo package and can be built or run without
installing additional dependencies.

## Projects

### 1. Key-Value Database

Path: [`key_value_database`](./key_value_database)

An interactive key-value database backed by an append-only `log.txt` file.
The database rebuilds its in-memory state from the log when it starts.

Supported commands:

| Command | Description |
| --- | --- |
| `set <key> <value>` | Add or replace a key-value pair |
| `get <key>` | Retrieve a value |
| `update <key> <value>` | Update an existing key |
| `delete <key>` | Delete an existing key |
| `display` | Display all stored pairs |
| `exit` | Exit the program |

Run it from its project directory so `log.txt` is created there:

```bash
cd key_value_database
cargo run
```

Values are currently read as a single whitespace-delimited argument, so they
cannot contain spaces.

### 2. Search Engine

Path: [`search_engine`](./search_engine)

A simple inverted-index search engine. At startup it scans the `.txt` files in
[`search_engine/data`](./search_engine/data), maps each word to the files that
contain it, and prints matching lines with their line numbers.

Run it from its project directory so the relative `data` path is available:

```bash
cd search_engine
cargo run
```

Searches are case-insensitive and match indexed words. Type `exit` to quit.
The included sample files cover algorithms, Python, and Rust.

### 3. Task Manager

Path: [`task_manager`](./task_manager)

A menu-driven in-memory task manager. Tasks contain an ID, name, description,
and completion status.

Menu options:

1. Add a task
2. View tasks
3. Mark a task as complete
4. Delete a task
5. Exit

Run it from its project directory:

```bash
cd task_manager
cargo run
```

Tasks are stored only while the program is running and are not persisted to a
file.

## Building

Build an individual project:

```bash
cargo build --manifest-path key_value_database/Cargo.toml
cargo build --manifest-path search_engine/Cargo.toml
cargo build --manifest-path task_manager/Cargo.toml
```

Check all projects without producing executables:

```bash
cargo check --manifest-path key_value_database/Cargo.toml
cargo check --manifest-path search_engine/Cargo.toml
cargo check --manifest-path task_manager/Cargo.toml
```

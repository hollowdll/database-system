// Entry point to database via terminal
// Under development!
// Currently in prototyping phase.
// Code will be improved later.

use std::{
    process,
    io,
};

fn exit_program() {
    println!("Exiting...");
    process::exit(0);
}

// TODO: Make this a config/build data structure in lib.rs
fn init() {
    use engine_core::Config;
    let config = Config::build();

    let help_message = "Write /help for all available commands";

    println!("\n{}\n", "Database Engine Project");
    println!("{}", help_message);

    // Program main loop
    loop {
        let mut input_command = String::new();

        println!("\n{}", "Enter a command:");
        io::stdin()
            .read_line(&mut input_command)
            .expect("Failed to read line");

        let input_command = input_command.trim();

        match input_command {
            "/help" => {
                println!("\n{}", "All available commands:");
                println!(
"
  /help                                   List all available commands
  /q                                      Quit
  (FOR TESTING) /test connection          Test connection to database manager
  (DISABLED) /connect                     Connect to database manager
  (DISABLED) /disconnect                  Disconnect from database manager
  (DISABLED) /databases                   List all databases
  (DISABLED) /create database [name]      Create a database with the given name
  (DISABLED) /delete database [nane]      Delete a database with the given name
  (DISABLED) /checkout database [name]    Switch currently active database
  (DISABLED) /create table [name]         Create a new table in the current database
  (DISABLED) /delete table [name]         Delete a table in the current database
  (DISABLED) /tables                      List all tables in the current database
  More commands in the future...
"
                );
                continue
            }
            "/q" => {
                exit_program()
            },
            "/test connection" => {
                println!("{:?}", config.database_manager())
            },
            _ => {
                println!("No such command found!");
                println!("{}", help_message);
                continue
            },
        }
    }
}

fn main() {
    init();
}

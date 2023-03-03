// Entry point to database via terminal
// Under development!
// Currently in prototyping phase.
// Code will be improved later.

use std::{
    process,
    io,
};
use engine_core;

fn exit_program() {
    println!("Exiting...");
    process::exit(0);
}

fn create_database() {

}

// TODO: Make this a config/build data structure in lib.rs
fn init() {
    let mut config = engine_core::Config::build();
    let database_manager = config.database_manager();

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
  /connect                                Connect to database manager
  /disconnect                             Disconnect from database manager
  /databases                              List all databases
  /create database                        Create a new database
  (DISABLED) /delete database [name]      Delete a database with the given name
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
                println!("{:?}", database_manager)
            },
            "/connect" => {
                database_manager.connect();
            },
            "/disconnect" => {
                database_manager.disconnect();
            },
            "/databases" => {
                println!(
                    "\n{}{}\n",
                    "Number of databases: ",
                    database_manager.database_count(),
                );
                // Also list all databases
            },
            "/create database" => {
                create_database();
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

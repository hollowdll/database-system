// Entry point to database via terminal
// Under development!
// Currently in prototyping phase.
// Code will be improved later.

use std::{
    process,
    io,
};
use engine_core::{
    self,
    DatabaseManager,
};

fn exit_program() {
    println!("Exiting...");
    process::exit(0);
}

// Consider moving to cli crate's lib.rs
fn prompt_database_creation(database_manager_mut: &mut DatabaseManager) {
    let mut database_name = String::new();
    let mut confirm = String::new();

    println!("\n{}", "Database name:");
    io::stdin()
        .read_line(&mut database_name)
        .expect("Failed to read line");

    let database_name = database_name.trim();

    println!("Confirm to create a new database named {}", database_name);
    println!("[Yes?]: y");
    io::stdin()
        .read_line(&mut confirm)
        .expect("Failed to read line");

    let confirm = confirm.trim();

    match confirm {
        "y" => {
            // Create database here
            database_manager_mut.create_database(database_name);

            println!("Created database: {}", database_name);
        },
        _ => {
            println!("Canceled database creation");
        },
    }
}

fn list_all_databases(database_manager: &DatabaseManager) {
    println!(
        "\n{}{}",
        "Number of databases: ",
        database_manager.databases().len(),
    );

    for i in database_manager.databases().iter() {
        println!("{:?}", i);
    }
}

// TODO: Make this a config/build data structure in lib.rs
fn init() {
    let mut config = engine_core::Config::build();

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
                println!("{:?}", config.database_manager())
            },
            "/connect" => {
                config.database_manager_mut().connect();
            },
            "/disconnect" => {
                config.database_manager_mut().disconnect();
            },
            "/databases" => {
                if config.database_manager().connected() {
                    list_all_databases(config.database_manager());
                } else {
                    println!("\
Not connected to database manager! \
Type /connect to connect to database manager."
                    );
                }
            },
            "/create database" => {
                if config.database_manager().connected() {
                    prompt_database_creation(config.database_manager_mut());
                } else {
                    println!("\
Not connected to database manager! \
Type /connect to connect to database manager."
                    );
                }
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

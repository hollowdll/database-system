// Entry point to database via terminal
// Under development!
// Currently in prototyping phase.
// Code will be improved later.

use std::{
    process,
    io,
};

// TODO: Make this a config/build data structure in lib.rs
// /help doesn't work yet
fn init() {
    let mut exit = false;
    let help_message = "Write /help for all available commands";

    println!("\n{}\n", "Database Engine Project");
    println!("{}\n", help_message);

    // Program main loop
    while !exit {
        let mut input_command = String::new();

        println!("Enter a command:");
        io::stdin()
            .read_line(&mut input_command)
            .expect("Failed to read line");

        let input_command = input_command.trim();

        match input_command {
            "/help" => {
                println!("List all available commands here\n");
                continue
            }
            "/q" => {
                println!("Exiting...");
                exit = true
            },
            _ => {
                println!("No such command found!");
                println!("{}\n", help_message);
                continue
            },
        }

        if exit {
            process::exit(0);
        }
    }
}

fn main() {
    init();
}

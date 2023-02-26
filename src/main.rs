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

    println!("\n\x1b[93mDatabase Engine Project\x1b[0m\n");
    println!("Write /help for all available commands\n");

    while !exit {
        let mut input_command = String::new();

        println!("Enter a command:");
        io::stdin()
            .read_line(&mut input_command)
            .expect("Failed to read line");

        let input_command = input_command.trim();

        match input_command {
            "/q" => {
                println!("Exiting...");
                exit = true
            },
            _ => {
                println!("No such command found!");
                println!("Write /help for all available commands\n");
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

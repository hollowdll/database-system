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
    let help_message = "Write /help for all available commands";

    println!("\n{}\n", "Database Engine Project");
    println!("{}\n", help_message);

    // Program main loop
    loop {
        let mut input_command = String::new();

        println!("Enter a command:");
        io::stdin()
            .read_line(&mut input_command)
            .expect("Failed to read line");

        let input_command = input_command.trim();

        match input_command {
            "/help" => {
                println!("\n{}", "All available commands:");
                println!(
"
  /help    List all available commands
  /q       Quit
"
                );
                continue
            }
            "/q" => {
                exit_program()
            },
            _ => {
                println!("No such command found!");
                println!("{}\n", help_message);
                continue
            },
        }
    }
}

fn main() {
    init();
}

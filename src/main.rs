mod command;
mod parser;
mod helper;
mod error;
mod command_impl;

use std::io;
use std::env;
use std::io::Write;
use crate::parser::{act_on_command, match_command};
use crate::command::RunResult;

fn main() {
    println!("            ╔══════════════════════════════════════════════════════════════╗
            ║                                                              ║
            ║  ██     ██ ██ ███    ██ ██    ██ ██   ██                     ║
            ║  ██     ██ ██ ████   ██ ██    ██  ██ ██                      ║
            ║  ██  █  ██ ██ ██ ██  ██ ██    ██   ███                       ║
            ║  ██ ███ ██ ██ ██  ██ ██ ██    ██  ██ ██                      ║
            ║   ███ ███  ██ ██   ████  ██████  ██   ██                     ║
            ║                                                              ║
            ║   ███████╗██╗  ██╗███████╗██╗     ██╗                        ║
            ║   ██╔════╝██║  ██║██╔════╝██║     ██║                        ║
            ║   ███████╗███████║█████╗  ██║     ██║                        ║
            ║   ╚════██║██╔══██║██╔══╝  ██║     ██║                        ║
            ║   ███████║██║  ██║███████╗███████╗███████╗                   ║
            ║   ╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝                   ║
            ║                                                              ║
            ║                 Linux-style shell for Windows                ║
            ║                                                              ║
            ╚══════════════════════════════════════════════════════════════╝");
    println!();
    let mut command: String = String::new();

    loop {
        let current_dir = env::current_dir().unwrap();
        print!("{}> ", current_dir.display());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).expect("Failed to read line");

        command = command.trim().to_string();

        let parsed_command = parser::command_parser(command.clone());
        
        let run_result = act_on_command(match_command(parsed_command));
        
        println!();
        io::stdout().flush().unwrap();
        helper::sleep();
        command.clear();
        
        if run_result == RunResult::Exit {
            break;
        }

    }

    println!("Exiting Winux Shell...");
}

mod command;
mod parser;
mod helper;
mod error;
mod command_impl;

use std::io;
use std::env;
use std::io::Write;
use crate::parser::{match_command, command_parser, build_command};
use crate::command::{Command, RunResult};

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

        let cmd: Command = match_command(command_parser(command.clone()));
        
        let built_command = build_command(cmd);
        let run_result = built_command.execute_struct().unwrap_or_else(|e| {
            e.message();
            RunResult::Continue
        });
        
        
        println!();
        io::stdout().flush().unwrap();
        command.clear();
        
        if run_result == RunResult::Exit {
            break;
        } else {
            helper::sleep();
        }

    }

    println!("Exiting Winux Shell...");
    helper::sleep();
}

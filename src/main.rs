use std::io;
use std::env;
use std::env::current_dir;
use std::io::Write;
use std::path::PathBuf;
// use std::path;
use std::fs;
use std::thread;
use std::time::Duration;

fn sleep(){
    thread::sleep(Duration::new(1,0));
}

fn resolve_path(path: String) -> PathBuf {
    return PathBuf::from(path);
}

fn handle_cd(path: &PathBuf) {
    return env::set_current_dir(path).unwrap_or_else(|_| print!("Could not find specified directory"))
}

fn handle_ls(current_path: String){
    let dir_list: Vec<String> = fs::read_dir(current_path.clone()).unwrap().map(|r| r.unwrap().file_name().to_str().unwrap().to_string()).collect();

    println!("Current directory: {}", current_path);
    dir_list.iter().for_each(|dir|{println!("- {}", dir);});
}

fn command_parser(command:String) -> (String, Vec<String>) {
    let parsed_command_raw: Vec<&str> = command.split(" ").collect();
    let mut parsed_command: Vec<String> = parsed_command_raw.iter().map(|x| x.to_string()).collect();

    return (parsed_command.remove(0), parsed_command)
}

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

        let parsed_command = command_parser(command.clone());

        command = parsed_command.0;
        let arguments = parsed_command.1;

        match command.as_str() {
            "pwd" => println!("{}", env::current_dir().unwrap().display()),
            "exit" => break,
            "clear" => print!("\x1B[2J\x1B[1;1H"),
            "cd" => handle_cd(&resolve_path(arguments.join(" ").to_string())),
            "ls" => handle_ls(current_dir.display().to_string()),
            "" => continue,
            _ => println!("Command not identified")
        }
        println!();
        io::stdout().flush().unwrap();
        sleep();
        command.clear();
    }

    println!("Exiting Winux Shell...");
    sleep();
}

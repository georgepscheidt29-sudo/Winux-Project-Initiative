use std::{env, fs, path};
use std::path::PathBuf;
use crate::error::{WinuxError};

pub enum Command {
    Pwd,
    Clear,
    Cd {path: PathBuf},
    Ls {args: Option<String>, path: Option<PathBuf>}, //TODO Implement args
    Exit,
    Unrecognized {cmd: String},
    Empty
}

pub struct Result {
    pub exec: (),
    pub run_status: i8
}

 impl Command {
    pub fn handle(&self) -> Result {
        match self {
            Command::Cd {path} => {
                Result{exec: env::set_current_dir(path).unwrap_or_else(|_| WinuxError::PathNotFound{path: path.to_path_buf()}.message()), run_status: 0}
            },

            Command::Pwd => {
                Result {exec: println!("Current Directory: {}", env::current_dir().unwrap().display()), run_status: 0}
            },

            Command::Clear => {
                Result{ exec: print!("\x1B[2J\x1B[1;1H"), run_status: 0}
            },

            Command::Ls { args, path } => {
                let current_path: PathBuf = match path {
                    Some(p) => match p.to_path_buf() {
                        Ok(pb) => pb,
                        Err(e) => WinuxError::SystemError { err }.message(),
                    },
                    None => env::current_dir()
                }

                let mut dir_list = Vec::new();
                Result{exec: 
                    match fs::read_dir(&current_path) {
                    Ok(entries) => {
                        for entry in entries {
                            match entry {
                                Ok(e) => dir_list.push(e.file_name().to_string_lossy().into_owned()),
                                Err(err) => WinuxError::SystemError { err }.message(),
                            }
                        }
                        println!("Path: {}", path::absolute(current_path).unwrap().display());
                        dir_list.iter().for_each(|dir| {println!("- {}", dir);});
                    }
                    Err(_) => {
                        WinuxError::PathNotFound { path: current_path.clone() }.message();
                    }
                },
                run_status: 0
                }

            },

            Command::Unrecognized {cmd} => {
                Result{exec: WinuxError::UnrecognizedCommand{cmd: cmd.to_string()}.message(), run_status: 0}
            },

            Command::Empty => {
                Result{exec: print!(""), run_status: 0}
            },

            Command::Exit => Result {exec: print!(""), run_status: 1}

        }
    }
}

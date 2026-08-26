use std::{env, fs, path};
use std::path::PathBuf;
use crate::error::WinuxError;

pub enum Command {
    Pwd,
    Clear,
    Cd {path: PathBuf},
    Ls {args: Option<String>, path: Option<PathBuf>}, //TODO Implement args
    Exit,
    Unrecognized {cmd: String},
    Empty
}

pub enum RunResult {
    Continue,
    Exit
}

impl RunResult {
    pub fn evaluate(cmd: Command) -> RunResult {
        if cmd == Command::Exit {
            return {RunResult::Exit}
        } else {
            return RunResult::Continue
        }
    }
}

 impl Command {
    pub fn handle(&self) -> Result<(), WinuxError> {
        match self {
            Command::Cd {path} => {
                env::set_current_dir(path)
                    .map_err(|e| WinuxError::SystemError{err: e} )?;
                Ok(())
            },

            Command::Pwd => {
                let cur_dir: PathBuf = env::current_dir()
                    .map_err(|e| WinuxError::SystemError{ err: e})?;
                
                Ok(println!("Current Directory: {}", cur_dir.display()))
            },

            Command::Clear => {
                Ok(print!("\x1B[2J\x1B[1;1H"))
            },

            Command::Ls { args, path } => {
                let current_path: PathBuf = match path {
                    Some(p) => p.to_path_buf(),
                    None => env::current_dir().map_err(|e| WinuxError::SystemError { err: e } )?,
                };
                
                let entries = fs::read_dir(&current_path)
                    .map_err(|e| WinuxError::SystemError { err: e } )?;

                let mut dir_list = Vec::new();
                
                for entry in entries {
                    let e = entry.map_err(|e| WinuxError::SystemError {err: e})?;
                    dir_list.push(e.file_name().to_string_lossy().into_owned());

                }
                Ok(())
            },

            Command::Unrecognized {cmd} => {
                Err( WinuxError::UnrecognizedCommand{ cmd: cmd.to_string() } )
            },

            Command::Empty => {
                Ok(())
            },

            Command::Exit =>{ 
                Ok(())
            }

        }
    }
}

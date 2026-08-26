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

pub struct RunResult {
    pub exec: (),
    pub run_status: i8
}

 impl Command {
    pub fn handle(&self) -> Result<(), WinuxError> {
        match self {
            Command::Cd {path} => {
                env::set_current_dir(path)
                    .map_err(|e| Err( WinuxError::SystemError{err: e} ));
                Ok(())
            },

            Command::Pwd => {
                let cur_dir: PathBuf = env::current_dir()
                    .map_err(WinuxError::SystemError{ err: e})?;
                
                Ok(println!("Current Directory: {}", cur_dir.display()))
            },

            Command::Clear => {
                Ok(print!("\x1B[2J\x1B[1;1H");)
            },

            Command::Ls { args, path } => {
                let current_path: PathBuf = match path {
                    Some(p) => p.to_path_buf(),
                    None => env::current_dir().map_err(|e| WinuxError::SystemError { err: e } ),
                };
                
                let entries = fs::read_dir(&current_path)
                    .map_err(|e| Err( WinuxError::SystemError { err: e } ));

                let mut dir_list = Vec::new();
                
                for entry in entries {
                    dir_list.push(entry.to_string_lossy().into_owned());

                    return Ok(())
                }

            },

            Command::Unrecognized {cmd} => {
                Err( WinuxError::UnrecognizedCommand{ cmd: cmd.to_string() } );
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

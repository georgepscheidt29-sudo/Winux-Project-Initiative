use std::fmt::Display;
use std::path::PathBuf;
use std::{io, path};

pub enum WinuxError {
    PathNotFound {path: PathBuf},
    SystemError{err: io::Error},
    DefaultError,
    UnrecognizedCommand{cmd: String}
}

impl WinuxError {
    pub fn message(&self) {
        match self {
            WinuxError::PathNotFound {path} => eprintln!("Could not find path {}", path::absolute(path).unwrap_or_else(|_| {path.to_path_buf()}).display()),
            WinuxError::SystemError {err} => eprintln!("{}", err),
            WinuxError::DefaultError => eprintln!("Process Failed, try again in a moment"),
            WinuxError::UnrecognizedCommand {cmd} => eprintln!("Unable to recognize command: {}", cmd)
        }
    }
}

// Deprecated, will possibly be reimplemented later
pub fn result_handler<T, E>(res: Result<T, E>) -> Option<T>
where E: Display,
{
    res.inspect_err(|e| eprintln!("Error encountered: {}", e))
       .ok()
}
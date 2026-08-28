use std::fmt::Display;
use std::path::PathBuf;
use std::{io, path};
use crate::error::WinuxError::UnrecognizedParameter;

pub enum WinuxError {
    PathNotFound {path: PathBuf},
    ArgumentNotExpected {cmd: String},
    ArgumentExpected {cmd: String},
    UnrecognizedParameter {param: String, cmd: String},
    SystemError {err: io::Error},
    DefaultError,
    UnrecognizedCommand {cmd: String}
}

impl WinuxError {
    pub fn message(&self) {
        match self {
            WinuxError::PathNotFound {path} => eprintln!("Could not find path {}", path::absolute(path).unwrap_or_else(|_| {path.to_path_buf()}).display()),
            WinuxError::SystemError {err} => eprintln!("{}", err),
            WinuxError::ArgumentNotExpected {cmd} => eprintln!("An argument was found where none was expected for command: {}", cmd),
            WinuxError::ArgumentExpected {cmd} => eprintln!("An argument was expected where none was found for command: {}", cmd),
            UnrecognizedParameter {param, cmd} => eprintln!("Unrecognized parameter: {} for command: {}", param, cmd),
            WinuxError::DefaultError => eprintln!("Process Failed, try again in a moment"),
            WinuxError::UnrecognizedCommand {cmd} => eprintln!("Unable to recognize command: {}", cmd)
        }
    }
}
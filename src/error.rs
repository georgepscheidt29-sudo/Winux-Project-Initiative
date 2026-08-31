use std::path::PathBuf;
use std::{io, path};

#[allow(dead_code)]
pub enum WinuxError { //TODO: Make new error for rm with message "Unable to remove file {file}, files deleted: {files vec}, see cause below:\n {Error}"
    PathNotFound {path: PathBuf},
    ArgumentNotExpected {cmd: String},
    ArgumentExpected {cmd: String},
    UnrecognizedParameter {param: String, cmd: String},
    SystemError {err: io::Error},
    DefaultError {msg: String},
    UnrecognizedCommand {cmd: String}
}

impl WinuxError {
    pub fn message(&self) {
        match self {
            WinuxError::PathNotFound {path} => eprintln!("Could not find path {}", path::absolute(path).unwrap_or_else(|_| {path.to_path_buf()}).display()),
            WinuxError::SystemError {err} => eprintln!("{}", err),
            WinuxError::ArgumentNotExpected {cmd} => eprintln!("An argument was found where none was expected for command: {}", cmd),
            WinuxError::ArgumentExpected {cmd} => eprintln!("An argument was expected where none was found for command: {}", cmd),
            WinuxError::UnrecognizedParameter {param, cmd} => eprintln!("Unrecognized parameter: {} for command: {}", param, cmd),
            WinuxError::DefaultError {msg} => eprintln!("{msg}"),
            WinuxError::UnrecognizedCommand {cmd} => eprintln!("Unable to recognize command: {}", cmd)
        }
    }
}
use std::fmt::Display;
use std::path::PathBuf;
use std::{io, path};

pub(crate) enum WinuxError {
    PathNotFound {path: PathBuf},
    SystemError{err: io::Error},
    UnrecognizedCommand{cmd: String}
}

impl WinuxError {
    pub(crate) fn message(&self) {
        match self {
            WinuxError::PathNotFound {path} => eprintln!("Could not find path {}", path::absolute(path).unwrap_or_else(|_| {path.to_path_buf()}).display()),
            WinuxError::SystemError {err} => eprintln!("{}", err),
            WinuxError::UnrecognizedCommand {cmd} => eprintln!("Unable to recognize command: {}", cmd)
        }
    }
}

// Deprecated
pub(crate) fn result_handler<T, E>(res: Result<T, E>) -> Option<T>
where E: Display,
{
    res.inspect_err(|e| eprintln!("Error encountered: {}", e))
       .ok()
}
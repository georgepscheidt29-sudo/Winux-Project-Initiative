use std::path::PathBuf;
use std::{io, path};
use crate::helper::print_vec_of_string;

#[allow(dead_code)]
pub enum WinuxError { 
    PathNotFound {path: PathBuf},
    SystemError {err: io::Error},
    DefaultError {msg: String},
    ExpectedDir,
    RmError {file: String, rm_files: Vec<String>, err: io::Error},
    RmRecursiveError {folder: String, err: io::Error},
    UnrecognizedCommand {cmd: String}
}

impl WinuxError {
    pub fn message(&self) {
        match self {
            WinuxError::PathNotFound {path} => eprintln!("Could not find path {}", path::absolute(path).unwrap_or_else(|_| {path.to_path_buf()}).display()),
            WinuxError::SystemError {err} => eprintln!("{}", err),
            WinuxError::DefaultError {msg} => eprintln!("{msg}"),
            WinuxError::ExpectedDir => eprintln!("Expected Directory, found other file type"),
            WinuxError::RmError {file, rm_files, err} => {
                eprintln!("Unable to remove file {}\n", file);
                if rm_files.is_empty() {
                    eprintln!();
                } else {
                    eprintln!("Successfully removed:");
                    print_vec_of_string(rm_files);
                }
                eprintln!("With root cause: {}", err)
            },
            WinuxError::RmRecursiveError {folder, err} => eprintln!("Error occurred while trying to delete folder: {} and its contents\n  With root cause: {}", folder, err),
            WinuxError::UnrecognizedCommand {cmd} => eprintln!("Unable to recognize command: {}", cmd)
        }
    }
}
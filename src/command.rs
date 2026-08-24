use std::{env, fs, path, io};
use std::path::PathBuf;
use crate::error::{result_handler, WinuxError};

pub enum Command {
    Pwd,
    Clear,
    Cd {path: PathBuf},
    Ls {args: Option<String>, path: Option<PathBuf>}, //TODO Implement args
    Exit,
    Unrecognized {cmd: String},
}

pub(crate) struct Result {
    pub(crate) exec: (),
    pub(crate) status: i8
}

pub(crate) fn handle_cd(path: &PathBuf) {
    env::set_current_dir(path).unwrap_or_else(|_| WinuxError::PathNotFound{path: path.to_path_buf()}.message())
}

pub(crate) fn handle_pwd() {
    println!("Current Directory: {}", env::current_dir().unwrap().display());
}

pub(crate) fn handle_ls(_args: Option<String>, path: Option<PathBuf>){

    let current_path: PathBuf = path.unwrap_or_else(|| env::current_dir().unwrap_or_default());

    let mut dir_list = Vec::new();

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
    }
}

pub(crate) fn handle_clear(){
    print!("\x1B[2J\x1B[1;1H")
}

pub(crate) fn handle_unrecognized(cmd: String) {
    WinuxError::UnrecognizedCommand{cmd}.message();
}
pub(crate) fn handle_exit() {
}
use std::{env, fs};
use std::path::PathBuf;
use crate::helper;
use crate::helper::resolve_path;

pub enum Command {
    Pwd,
    Clear,
    Cd {path: PathBuf},
    Ls {args: Option<String>, path: Option<PathBuf>},
    Exit,
    Unrecognized,
}

pub(crate) struct Result {
    pub(crate) exec: (),
    pub(crate) status: i8
}

pub(crate) fn handle_cd(path: &PathBuf) {
    env::set_current_dir(path).unwrap_or_else(|_| print!("Could not find specified directory"))
}

pub(crate) fn handle_pwd() {
    println!("Current Directory: {}", env::current_dir().unwrap().display());
}

pub(crate) fn handle_ls(args: Option<String>, path: Option<PathBuf>){
    let mut current_path = PathBuf::new();
    match path {
        Some(path) => {
            current_path = PathBuf::from(path);
        }
        None => {current_path = env::current_dir().unwrap();}
    }

    let dir_list: Vec<String> = fs::read_dir(current_path.clone()).unwrap().map(|r| r.unwrap().file_name().to_str().unwrap().to_string()).collect();

    println!("Path: {}", current_path.display());
    dir_list.iter().for_each(|dir|{println!("- {}", dir);});
}

pub(crate) fn handle_clear(){
    print!("\x1B[2J\x1B[1;1H")
}

pub(crate) fn handle_unrecognized() {
    print!("Unrecognized command");
}
pub(crate) fn handle_exit() {
    return
}
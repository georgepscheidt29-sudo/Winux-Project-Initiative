use std::{env, fs, path};
use std::env::current_dir;
use std::path::PathBuf;

pub enum Command {
    Pwd,
    Clear,
    Cd {path: PathBuf},
    Ls {args: Option<String>, path: Option<PathBuf>}, //TODO Implement args
    Exit,
    Unrecognized, //TODO Implement specified command on final message, should read Could not recognize command {attempted command}
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

    let current_path: PathBuf = path.unwrap_or_else(|| env::current_dir().unwrap_or_default());


    let dir_list: Vec<String> = fs::read_dir(current_path.clone()).unwrap().map(|r| r.unwrap().file_name().to_str().unwrap().to_string()).collect(); //TODO Fix error handling, unwrap_or_else does not handle panics

    println!("Path: {}", path::absolute(current_path).unwrap().display());
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
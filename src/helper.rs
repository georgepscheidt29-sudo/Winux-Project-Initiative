use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

pub fn sleep(){
    thread::sleep(Duration::from_millis(500));
}

pub fn resolve_path_or_none(path: Option<&String>) -> Option<PathBuf> {
    match path {
        Some(path) => Option::from(PathBuf::from(path)),
        None => {None}
    }
}

// First item of tuple is always args, second is always path - Used for functions that can take paths, arguments, both or neither
pub fn resolve_args_and_path(args: &[String]) -> (Option<String>, Option<String>) {
    match args.len() {
        0 => (None, None),

        1 => if path_or_args(&args[0]) {
            (Option::from(args[0].clone()), None)

        } else {
            (None, Option::from(args[0].clone()))

        },
        _ => if path_or_args(&args[0]) {
            (Option::from(args[0].clone()), Option::from(args[1].clone()))

        } else {
            (Option::from(args[1].clone()), Option::from(args[0].clone())) 

        }
    }
}

// Returns true if it is an argmument starting with -, else returns false, considering it as path or file name
fn path_or_args(param: &str) -> bool {
    param.trim().starts_with("-")
}
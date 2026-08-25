use std::path::PathBuf;
use std::thread;
use std::time::Duration;

pub fn sleep(){
    thread::sleep(Duration::new(1,0));
}

pub fn resolve_path(path: Option<&String>) -> Option<PathBuf> {
    match path {
        Some(path) => Option::from(PathBuf::from(path)),
        None => {None}
    }
}
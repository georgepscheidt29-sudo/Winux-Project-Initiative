use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use std::fs::{DirEntry, ReadDir};

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

// OS conditional compilation, this project is overall meant for windows, but implementing this also allows me to begin thinking about conditional comp for future projects.

// Windows Implementation
#[cfg(target_os = "windows")]
fn is_hidden(entry: &DirEntry) -> bool {
    use std::os::windows::fs::MetadataExt;

    const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;

    if let Ok(metadata) = entry.metadata() {
        return (metadata.file_attributes() & FILE_ATTRIBUTE_HIDDEN) != 0;
    }
    false
}

// Unix/Linux/macOS Implementation
#[cfg(not(target_os = "windows"))]
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .is_some_and(|s| s.starts_with('.'))
}


pub fn filter_hidden_files(entries: ReadDir) -> Vec<std::io::Result<DirEntry>> {
    entries
        .filter(|entry_result| {
            match entry_result {
                Ok(entry) => !is_hidden(entry),
                Err(_) => true, 
            }
        })
        .collect()
}
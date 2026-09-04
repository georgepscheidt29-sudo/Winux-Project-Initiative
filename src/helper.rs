use std::path::{Path, PathBuf};
use std::{thread};
use std::time::Duration;
use std::fs;
use std::fs::{metadata, DirEntry, ReadDir};
use std::io::{stdin, Error};
use chrono::DateTime;
use crate::error::WinuxError;
use pad::PadStr;
use crate::command_impl::file_type::match_file_type;

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

// Essentially equal to function above, but specifically built for commands that take args and possibly more than one path
pub fn resolve_args_and_path_list(args: &[String]) -> (Option<String>, Option<Vec<String>>) {
    let mut param: Option<String> = None;
    let mut path_list: Option<Vec<String>> = Some(Vec::new());

    if args.is_empty() {
        return (None, None);
    } else {
        for arg in args {
            if path_or_args(arg) {
                param = Some(arg.clone());
            } else {
                path_list.as_mut().unwrap().push(arg.clone());
            }
        }
    }

    (param, path_list)
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

pub fn parse_read_dir(entries: ReadDir) -> Vec<std::io::Result<DirEntry>> {
    let mut result: Vec<std::io::Result<DirEntry>> = Vec::new();
    for x in entries {
        result.push(x);
    }

    result
}

pub fn build_metadata(entries: Vec<Result<DirEntry, Error>>) -> Result<String, WinuxError> {

    let mut resulting_string = String::new();

    resulting_string.push_str(&format!("{:<10}", "Size"));
    resulting_string.push_str(&format!("{:<8}", "Type"));
    resulting_string.push_str(&format!("{:<35}", "File name"));
    resulting_string.push_str(&format!("{:<15}", "Created at"));
    resulting_string.push_str("Last modified at\n");

    for entry in entries {
        let usable_entry = entry.map_err(|e| WinuxError::SystemError {err: e })?;
        let metadata = metadata(usable_entry.path());
        let usable_metadata = metadata.map_err(|e| WinuxError::SystemError {err: e})?;

        let file_size = usable_metadata.len();
        let file_type = match_file_type(&usable_metadata).to_string();
        let name = usable_entry.file_name();
        let created_at = usable_metadata.created().map_err(|e| WinuxError::SystemError {err: e})?;
        let modified_at = usable_metadata.modified().map_err(|e| WinuxError::SystemError {err: e})?;

        let size_in_kb: f64 = file_size as f64 / 1024.0;
        let mut readable_size: String = size_in_kb.to_string().with_exact_width(4).to_string();
        readable_size.push_str(" KB");
        let readable_created = DateTime::<chrono::Local>::from(created_at);
        let readable_modified = DateTime::<chrono::Local>::from(modified_at);

        resulting_string.push_str(&readable_size.with_exact_width(10).to_string());
        resulting_string.push_str(&file_type.with_exact_width(8).to_string());
        resulting_string.push_str(&name.to_string_lossy().with_exact_width(35).to_string());
        resulting_string.push_str(&readable_created.format("%b %d %H:%M").to_string().with_exact_width(15).to_string());
        resulting_string.push_str(&format!("{}\n", readable_modified.format("%b %d %H:%M")));

    }

    Ok(resulting_string)
}

pub fn await_user_approval_y_or_n(prompt: String) -> Result<bool, WinuxError> {
    let mut answer: String = String::new();

    println!("{}, (y/n)", prompt);
    stdin().read_line(&mut answer).map_err(|e| WinuxError::SystemError {err: e })?;

    match answer.trim() {
        "y" => Ok(true),
        "yes" => Ok(true),
        "" => Ok(true),
        _ => Ok(false),
    }
}

pub fn print_vec_of_string(vec_to_print: &Vec<String>) {
    for s in vec_to_print {
        eprintln!("{}\n", s);
    }
}

pub fn rm_file(
    confirm: bool,
    force: bool,
    path: &PathBuf,
    rm_files: &[String],
) -> Result<bool, WinuxError> {

    let metadata = fs::symlink_metadata(path)
        .map_err(|e| WinuxError::SystemError { err: e })?;

    if metadata.is_dir() {
        return Err(WinuxError::DefaultError {
            msg: "Path provided was a directory, use -r if this was intentional".to_string()
        });
    }

    if confirm {
        let msg = format!("Remove {}?", path.display());

        if !await_user_approval_y_or_n(msg)? {
            return Ok(false);
        }
    }

    match fs::remove_file(path) {
        Ok(_) => Ok(true),

        Err(e) if force && e.kind() == std::io::ErrorKind::NotFound => {
            Ok(false)
        }

        Err(e) => Err(WinuxError::RmError {
            file: path.to_string_lossy().to_string(),
            rm_files: rm_files.to_owned(),
            err: e,
        }),
    }
}

pub fn resolve_path_vec(unresolved_vec: Option<Vec<String>>) -> Option<Vec<PathBuf>> {
    let mut resolved_path_vec: Vec<PathBuf> = Vec::new();

    match unresolved_vec {
        Some(vec) => {
            for path in vec {
                resolved_path_vec.push(PathBuf::from(path));
            }
        },
        None => return None,
    }

    Some(resolved_path_vec)
}

pub fn copy_file(file: PathBuf, destination: PathBuf) -> Result<PathBuf, WinuxError> {

    fs::copy(&file, &destination).map_err(|e| WinuxError::SystemError {err: e })?;
    
    if file.is_dir {
        return destination.push(file);
    } else {
        return destination;
    }

    Ok(())
}

use std::{env, fs, path};
use std::fs::{DirEntry, File, FileTimes, symlink_metadata};
use std::path::PathBuf;
use std::time::SystemTime;
use crate::run_result::{RunResult};
use crate::command_impl::command_builder::Executable;
use crate::error::WinuxError;
use crate::helper::{build_metadata, filter_hidden_files, parse_read_dir, rm_file};

// +===== PWD Implementation =====+

pub(crate) struct PwdStruct {}

impl Executable for PwdStruct {
    fn execute(&self) -> Result<RunResult,WinuxError> {
        let cur_dir: PathBuf = env::current_dir()
            .map_err(|e| WinuxError::SystemError { err: e })?;

        println!("Current Directory: {}", cur_dir.display());

        Ok(RunResult::Continue)
    }
}

// ===== CD Implementation =====

pub(crate) struct CdStruct {
    pub(crate) path: Option<PathBuf>
}

impl Executable for CdStruct {
    fn execute(&self) -> Result<RunResult,WinuxError> {
        let defined_path: PathBuf = match &self.path {
            Some(p) => {p.to_owned()},
            None => {env::current_dir().map_err(|e| WinuxError::SystemError { err: e })?}
        };

        env::set_current_dir(defined_path)
            .map_err(|e| WinuxError::SystemError { err: e })?;
        Ok(RunResult::Continue)
    }
}

// ===== LS Implementation =====

pub struct LsStruct {
    pub(crate) args: Option<String>,
    pub(crate) path: Option<PathBuf>
}


impl Executable for LsStruct {
    fn execute(&self) -> Result<RunResult,WinuxError> {
        let current_path: PathBuf = match &self.path {
            Some(p) => p.to_owned(),

            None => env::current_dir().map_err(|e| WinuxError::SystemError { err: e } )?,

        };

        let args = match &self.args {
            Some(a) => a.to_owned(),

            None => String::from("")
        };

        let entries = fs::read_dir(&current_path)
            .map_err(|e| WinuxError::SystemError { err: e } )?;


        
        let mut dir_list = Vec::new();

        let mut string_to_print: String = String::new();

        let usable_entries: Vec<std::io::Result<DirEntry>> = if args.contains("a") {
            dir_list.push(String::from(".."));
            dir_list.push(String::from("."));
            parse_read_dir(entries)

        } else {
            filter_hidden_files(entries)
        };

        if args.contains("l") {
            string_to_print = build_metadata(usable_entries)?;

        } else {
            for entry in usable_entries {
                let e = entry.map_err(|e| WinuxError::SystemError {err: e})?;
                dir_list.push(e.file_name().to_string_lossy().into_owned());
            }

            for i in 0..dir_list.len() {
                if i%5 != 0 || i == 0 {
                    string_to_print.push_str(&format!("- {}\t", dir_list[i]));

                } else if i != 0 {
                    string_to_print.push_str(&format!("- {}\n\n", dir_list[i]));

                }
            }
        }

        println!("{}", current_path.display());
        println!("\n{}",string_to_print);

        Ok(RunResult::Continue)
    }
}

// ===== MKDIR Implementation =====

pub struct MkDirStruct {
    pub(crate) args: Option<String>,
    pub(crate) path: Option<PathBuf>,

}

impl Executable for MkDirStruct {
    fn execute(&self) -> Result<RunResult,WinuxError> {
        let args = match &self.args {
            Some(a) => a.to_owned(),
            None => String::from("")
        };

        let path = match &self.path {
            Some(p) => p.to_owned(),
            None => return Err(WinuxError::DefaultError {msg: "Command MkDir expects a directory name/path, none specified".to_string()}),
        };

        if args.contains("p") {
            fs::create_dir_all(&path).map_err(|e| WinuxError::SystemError { err: e })?;
            Ok(RunResult::Continue)

        } else {
            fs::create_dir(&path).map_err(|e| WinuxError::SystemError { err: e })?;
            Ok(RunResult::Continue)
        }

    }
}

// ===== TOUCH Implementation =====

pub struct TouchStruct {
    pub(crate) path: Option<PathBuf>,

}

impl Executable for TouchStruct {
    fn execute(&self) -> Result<RunResult,WinuxError> {
        let path = match &self.path {
            Some(p) => p.to_owned(),
            None => return Err(WinuxError::DefaultError {msg: "Command Touch expects a file name/path, none specified".to_string()}),
        };

        if path.exists() {
            let current_time = FileTimes::new()
                .set_modified(SystemTime::now())
                .set_accessed(SystemTime::now());

            let file = File::open(&path).map_err(|e| WinuxError::SystemError { err: e })?;
            file.set_times(current_time).map_err(|e| WinuxError::SystemError { err: e })?;

        } else {
            File::create(&path).map_err(|e| WinuxError::SystemError { err: e })?;
        }

        Ok(RunResult::Continue)
    }
}

// ===== RM Implementation =====

pub struct RmStruct {
    pub(crate) args: Option<String>,
    pub(crate) path: Option<Vec<PathBuf>>,
}

impl Executable for RmStruct {
    fn execute(&self) -> Result<RunResult,WinuxError> {
        let args = match &self.args {
            Some(a) => a.to_owned(),
            None => String::from("")
        };

        let path_list = match &self.path {
            Some(p) => p.to_owned(),
            None => return Err(WinuxError::DefaultError {msg: "Command Rm expects a file name/path, none specified".to_string()}),
        };

        let force = args.contains("f");
        let confirm = args.contains("i");
        let recursive = args.contains("r");

        let mut removed_files: Vec<String> = Vec::new();

        for path in path_list {
            if !recursive {
                if rm_file(confirm, force, &path, &removed_files)? {
                    removed_files.push(
                        path::absolute(path)
                            .map_err(|e| WinuxError::SystemError { err: e })?
                            .to_string_lossy()
                            .into_owned()
                    );
                }
            } else {
                handle_removal_recursive(confirm, force, &path, &removed_files)?;

                removed_files.push(
                    path::absolute(path)
                        .map_err(|e| WinuxError::SystemError { err: e })?
                        .to_string_lossy()
                        .into_owned()
                );
            }
        }

        Ok(RunResult::Continue)
    }
}

fn handle_removal_recursive(
    confirm: bool,
    force: bool,
    path: &PathBuf,
    rm_files: &[String],
) -> Result<bool, WinuxError> {

    let dir_entries = parse_read_dir(
        fs::read_dir(path)
            .map_err(|e| WinuxError::SystemError { err: e })?
    );

    for entry in dir_entries {
        let e = entry
            .map_err(|e| WinuxError::SystemError { err: e })?
            .path();

        let metadata = symlink_metadata(&e)
            .map_err(|e| WinuxError::SystemError { err: e })?;

        if metadata.is_dir() {
            handle_removal_recursive(confirm, force, &e, rm_files)?;
        } else {
            rm_file(confirm, force, &e, rm_files)?;
        }
    }

    fs::remove_dir(path)
        .map_err(|e| WinuxError::SystemError { err: e })?;

    Ok(true)
}


// ===== CP Implementation =====

pub struct CpStruct {
    pub(crate) args: Option<String>,
    pub(crate) paths: Option<Vec<PathBuf>>,
}

impl Executable for CpStruct {
    fn execute(&self) -> Result<RunResult,WinuxError> {
        let args = match &self.args {
            Some(a) => a.to_owned(),
            None => String::from("")
        };
        
        let paths = match &self.paths {
            Some(p) => p.to_owned(),
            None => return Err(WinuxError::DefaultError {msg: "Command Cp expects at least 2 paths, none were provided".to_string()})
        };
        
        if paths.len() < 2 { 
            return Err(WinuxError::DefaultError {msg: "Command Cp expects at least 2 paths".to_string()})
        }
        
        
        
        Ok(RunResult::Continue)
    }
}
use std::{env, fs};
use std::fs::DirEntry;
use std::path::PathBuf;
use crate::run_result::{RunResult};
use crate::command_impl::command_builder::Executable;
use crate::error::WinuxError;
use crate::helper::{build_metadata, filter_hidden_files, parse_read_dir};

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

// +===== CD Implementation =====+

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

// +===== LS Implementation =====+

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

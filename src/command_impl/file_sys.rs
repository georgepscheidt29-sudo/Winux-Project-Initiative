use std::{env, fs};
use std::path::PathBuf;
use crate::command::{RunResult};
use crate::command_impl::command_builder::Executable;
use crate::error::WinuxError;

pub(crate) struct PwdStruct {}

impl Executable for PwdStruct {
    fn execute(&self) -> Result<RunResult,WinuxError> {
        let cur_dir: PathBuf = env::current_dir()
            .map_err(|e| WinuxError::SystemError { err: e })?;

        println!("Current Directory: {}", cur_dir.display());

        Ok(RunResult::Continue)
    }
}

pub(crate) struct CdStruct {
    pub(crate) path: Option<PathBuf>
}

impl Executable for CdStruct {
    fn execute(&self) -> Result<RunResult,WinuxError> {
        let defined_path: PathBuf;
        match &self.path {
            Some(p) => {defined_path = p.to_owned();},
            None => {defined_path = env::current_dir().map_err(|e| WinuxError::SystemError { err: e })?;}
        }

        env::set_current_dir(defined_path)
            .map_err(|e| WinuxError::SystemError { err: e })?;
        Ok(RunResult::Continue)
    }
}

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

        let entries = fs::read_dir(&current_path)
            .map_err(|e| WinuxError::SystemError { err: e } )?;

        let mut dir_list = Vec::new();

        for entry in entries {
            let e = entry.map_err(|e| WinuxError::SystemError {err: e})?;
            dir_list.push(e.file_name().to_string_lossy().into_owned());

        }
        println!("{}", current_path.display());
        dir_list.iter().for_each( |d|
            println!("- {}", d)
        );

        Ok(RunResult::Continue)
    }
}

use crate::run_result::{RunResult};
use crate::command_impl::{general_use, file_sys};
use crate::error::WinuxError;

pub enum BuiltCommand { //TODO: Make both Command and BuiltCommand into a single enum
    BuiltPwd(file_sys::PwdStruct),
    BuiltCd(file_sys::CdStruct),
    BuiltLs(file_sys::LsStruct),
    BuiltClear(general_use::ClearStruct),
    BuiltExit(general_use::ExitStruct),
    BuiltUnrecognized(general_use::UnrecognisedStruct),
    BuiltEmpty(general_use::EmptyStruct),

}

impl BuiltCommand {
    pub fn execute_struct(&self) -> Result<RunResult, WinuxError> {
        match self {
            BuiltCommand::BuiltPwd(c) => {c.execute()},
            BuiltCommand::BuiltCd(c) => {c.execute()},
            BuiltCommand::BuiltLs(c) => {c.execute()},
            BuiltCommand::BuiltClear(c) => {c.execute()},
            BuiltCommand::BuiltExit(c) => {c.execute()},
            BuiltCommand::BuiltUnrecognized(c) => {c.execute()},
            BuiltCommand::BuiltEmpty(c) => {c.execute()},
        }
    }
}

pub trait Executable {
    fn execute(&self) -> Result<RunResult,WinuxError>;
}

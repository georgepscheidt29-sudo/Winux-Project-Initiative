use crate::run_result::{RunResult};
use crate::command_impl::{general_use, file_sys};
use crate::error::WinuxError;

// TODO: Improve current {command} Implementation comments

pub enum BuiltCommand {

    // File System
    Pwd(file_sys::PwdStruct),
    Cd(file_sys::CdStruct),
    Ls(file_sys::LsStruct),
    MkDir(file_sys::MkDirStruct),
    Touch(file_sys::TouchStruct),
    Rm(file_sys::RmStruct),
    Cp(file_sys::CpStruct),

    // General Commands
    Clear(general_use::ClearStruct),
    Exit(general_use::ExitStruct),
    Unrecognized(general_use::UnrecognisedStruct),
    Empty(general_use::EmptyStruct),
    Test(general_use::TestStruct),

}

impl BuiltCommand {
    pub fn execute_struct(&self) -> Result<RunResult, WinuxError> {
        match self {
            // File System
            BuiltCommand::Pwd(c) => {c.execute()},
            BuiltCommand::Cd(c) => {c.execute()},
            BuiltCommand::Ls(c) => {c.execute()},
            BuiltCommand::MkDir(c) => {c.execute()},
            BuiltCommand::Touch(c) => {c.execute()},
            BuiltCommand::Rm(c) => {c.execute()},
            BuiltCommand::Cp(_c) => {unimplemented!()},

            // General Commands
            BuiltCommand::Clear(c) => {c.execute()},
            BuiltCommand::Exit(c) => {c.execute()},
            BuiltCommand::Unrecognized(c) => {c.execute()},
            BuiltCommand::Empty(c) => {c.execute()},
            BuiltCommand::Test(c) => {c.execute()}, // Here only to test behaviors during development, the struct and .execute will more often then not be empty when pushed to github
        }
    }
}

pub trait Executable {
    fn execute(&self) -> Result<RunResult,WinuxError>;
}

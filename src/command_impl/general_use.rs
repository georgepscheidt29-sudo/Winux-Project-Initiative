use crate::run_result::{RunResult};
use crate::command_impl::command_builder::Executable;
use crate::error::WinuxError;

pub struct ExitStruct {}

impl Executable for ExitStruct {
    fn execute(&self) -> Result<RunResult, WinuxError> {   
        Ok(RunResult::Exit)
    }
}

pub struct ClearStruct{}

impl Executable for ClearStruct {
    fn execute(&self) -> Result<RunResult, WinuxError> {
        print!("\x1B[2J\x1B[1;1H");
        Ok(RunResult::Continue)
    }
}

pub struct EmptyStruct {}

impl Executable for EmptyStruct {
    fn execute(&self) -> Result<RunResult, WinuxError> {
        Ok(RunResult::Continue)
    }
}

pub struct UnrecognisedStruct {
    pub(crate) cmd: String
}

impl Executable for UnrecognisedStruct {
    fn execute(&self) -> Result<RunResult, WinuxError> {
        Err(WinuxError::UnrecognizedCommand {cmd: self.cmd.to_owned()})
    }
}

pub struct TestStruct {}

impl Executable for TestStruct {
    fn execute(&self) -> Result<RunResult, WinuxError> {
        Ok(RunResult::Continue)
    }
}

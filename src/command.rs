use std::{env, fs};
use std::path::PathBuf;
use crate::error::WinuxError;

#[derive(PartialEq)]
pub enum Command {
    Pwd,
    Clear,
    Cd {args: Vec<String>},
    Ls {args: Vec<String>},
    Exit,
    Unrecognized {cmd: String},
    Empty
}

#[derive(PartialEq)]
pub enum RunResult {
    Continue,
    Exit
}

impl RunResult {
    pub fn evaluate_cmd(cmd: Command) -> RunResult {
        if cmd == Command::Exit {
            return RunResult::Exit
        } else {
            return RunResult::Continue
        }
    }
}

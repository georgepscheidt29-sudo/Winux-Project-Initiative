use std::env::current_dir;
use crate::command::{Command, RunResult};
use crate::helper::resolve_path;
use crate::error::WinuxError;

pub fn command_parser(command:String) -> (String, Vec<String>) {
    let parsed_command_raw: Vec<&str> = command.split(" ").collect();
    let mut parsed_command: Vec<String> = parsed_command_raw.iter().map(|x| x.to_string()).collect();

    (parsed_command.remove(0), parsed_command)
}

pub fn match_command( raw_cmd: (String, Vec<String>)) -> Command {
    let cmd = raw_cmd.0.to_lowercase();
    let params = raw_cmd.1;

    match cmd.as_str() {
        "pwd" => Command::Pwd,
        "cd" => Command::Cd {path: resolve_path(params.first()).unwrap_or_else(|| current_dir().unwrap())},
        "ls" => Command::Ls {args: params.first().cloned(), path: resolve_path(params.last()) },
        "clear" => Command::Clear,
        "exit" => Command::Exit,
        "" => Command::Empty,
        _ => Command::Unrecognized{ cmd: cmd.to_string()}
    }
}

pub fn match_args(params: Vec<String>) {

}

impl Command {
    pub fn to_run_result(&self) -> RunResult {
        if *self == Command::Exit {
            RunResult::Exit
        } else {
            RunResult::Continue
        }

    }
}

impl WinuxError {
    pub fn to_run_result(&self) -> RunResult {
        return RunResult::Continue
    }
}

pub fn act_on_command(cmd: Result<Command, WinuxError>) -> RunResult{
    match cmd {
        Ok(c) => {
            c.handle();
            return c.to_run_result()
        },
        Err(we) => {
            we.message();
            return we.to_run_result()
        }
    }
}
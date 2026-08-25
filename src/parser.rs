use std::env::current_dir;
use crate::command::{Command, Result};
use crate::command;
use crate::helper::resolve_path;

pub(crate) fn command_parser(command:String) -> (String, Vec<String>) {
    let parsed_command_raw: Vec<&str> = command.split(" ").collect();
    let mut parsed_command: Vec<String> = parsed_command_raw.iter().map(|x| x.to_string()).collect();

    (parsed_command.remove(0), parsed_command)
}

pub(crate) fn match_command( raw_cmd: (String, Vec<String>)) -> Command {
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

pub(crate) fn act_on_command(cmd: Command) -> Result{
    match cmd {
        Command::Pwd => Result {exec: command::handle_pwd(), status: 0},
        Command::Cd {path} => Result {exec: command::handle_cd(&path), status: 0},
        Command::Ls {args, path} => Result {exec: command::handle_ls(args, path), status: 0},
        Command::Clear => Result {exec: command::handle_clear(), status: 0},
        Command::Exit => Result {exec: command::handle_exit(), status: 1},
        Command::Empty => Result {exec: command::handle_empty(), status: 0},
        Command::Unrecognized {cmd} => Result {exec: command::handle_unrecognized(cmd), status: 0}
    }
}
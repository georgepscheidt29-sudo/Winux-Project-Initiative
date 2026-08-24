use std::env::current_dir;
use crate::command::{handle_ls, Command, Result, handle_pwd, handle_cd, handle_exit, handle_clear, handle_unrecognized};
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
        "cd" => Command::Cd {path: resolve_path(params.first().clone()).unwrap_or_else(|| current_dir().unwrap())},
        "ls" => Command::Ls {args: params.first().cloned(), path: resolve_path(params.last().clone()) },
        "clear" => Command::Clear,
        "exit" => Command::Exit,
        _ => Command::Unrecognized
    }
}

pub(crate) fn act_on_command(cmd: Command) -> Result{
    match cmd {
        Command::Pwd => Result {exec: handle_pwd(), status: 0},
        Command::Cd {path} => Result {exec: handle_cd(&path), status: 0},
        Command::Ls {args, path} => Result {exec: handle_ls(args, path), status: 0},
        Command::Clear => Result {exec: handle_clear(), status: 0},
        Command::Exit => Result {exec: handle_exit(), status: 1},
        Command::Unrecognized => Result {exec: handle_unrecognized(), status: 0}
    }
}
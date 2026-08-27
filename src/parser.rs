use std::path::PathBuf;
use crate::command::{Command, RunResult};
use crate::command_impl::command_builder::BuiltCommand;
use crate::command_impl::command_builder::BuiltCommand::BuiltClear;
use crate::command_impl::file_sys::{CdStruct, LsStruct, PwdStruct};
use crate::command_impl::general_use::{ClearStruct, EmptyStruct, ExitStruct, UnrecognisedStruct};
use crate::error::WinuxError;
use crate::helper::{resolve_args_and_path, resolve_path_or_none};

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
        "cd" => Command::Cd {args: params},
        "ls" => Command::Ls {args:  params},
        "clear" => Command::Clear,
        "exit" => Command::Exit,
        "" => Command::Empty,
        _ => Command::Unrecognized{ cmd: cmd.to_string()}
    }
}

pub fn build_command(cmd: Command) -> BuiltCommand {
    match cmd {
        Command::Pwd => BuiltCommand::BuiltPwd(PwdStruct {}),

        Command::Cd {args} => BuiltCommand::BuiltCd(CdStruct {
            path: resolve_path_or_none(args.first() )}),

        Command::Ls {args} => {
            let possible_args: Option<String>= resolve_args_and_path(&args).0;
            let possible_path: Option<String> = resolve_args_and_path(&args).1;
            let resolved_path: Option<PathBuf>;
            match possible_path {
                Some(p) => { resolved_path = resolve_path_or_none(Some(&p)); },
                None => { resolved_path = None; }
            }

            BuiltCommand::BuiltLs(LsStruct {
                args: possible_args,
                path: resolved_path,
            }) },

        Command::Clear => {BuiltClear(ClearStruct {})},

        Command::Exit => BuiltCommand::BuiltExit(ExitStruct {}),

        Command::Empty => BuiltCommand::BuiltEmpty(EmptyStruct {}),

        Command::Unrecognized {cmd} => BuiltCommand::BuiltUnrecognized(UnrecognisedStruct{cmd: cmd.to_string()}),
    }
}
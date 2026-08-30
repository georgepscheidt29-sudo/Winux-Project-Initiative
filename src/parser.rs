use std::path::PathBuf;
use crate::command_impl::command_builder::BuiltCommand;
use crate::command_impl::file_sys::{CdStruct, LsStruct, MkDirStruct, PwdStruct, TouchStruct};
use crate::command_impl::general_use::{ClearStruct, EmptyStruct, ExitStruct, TestStruct, UnrecognisedStruct};
use crate::helper::{resolve_args_and_path, resolve_path_or_none};

pub fn command_parser(command:String) -> (String, Vec<String>) {
    let parsed_command_raw: Vec<&str> = command.split(" ").collect();
    let mut parsed_command: Vec<String> = parsed_command_raw.iter().map(|x| x.to_string()).collect();

    (parsed_command.remove(0), parsed_command)
}

pub fn build_command( raw_cmd: (String, Vec<String>) ) -> BuiltCommand {
    let cmd = raw_cmd.0.to_lowercase();
    let params = raw_cmd.1;
    
    match cmd.as_str() {
        "pwd" => BuiltCommand::Pwd(PwdStruct {}),

        "cd" => BuiltCommand::Cd(CdStruct {
            path: resolve_path_or_none(params.first() )}),

        "ls" => {
            let resolved_args = resolve_args_and_path(&params);
            let possible_args: Option<String>= resolved_args.0;
            let possible_path: Option<String> = resolved_args.1;

            let resolved_path: Option<PathBuf> = match possible_path {
                Some(p) => { resolve_path_or_none(Some(&p)) },
                None => { None }
            };

            BuiltCommand::Ls(LsStruct {
                args: possible_args,
                path: resolved_path,
            }) },

        "clear" => {BuiltCommand::Clear(ClearStruct {})},

        "exit" => BuiltCommand::Exit(ExitStruct {}),

        "mkdir" => {
            let resolved_args = resolve_args_and_path(&params);
            let possible_args: Option<String>= resolved_args.0;
            let possible_path: Option<String> = resolved_args.1;

            let resolved_path: Option<PathBuf> = match possible_path {
                Some(p) => { resolve_path_or_none(Some(&p)) },
                None => { None }
            };

            BuiltCommand::MkDir(MkDirStruct {
                args: possible_args,
                path: resolved_path
            })
        },

        "touch" => {
            BuiltCommand::Touch(TouchStruct {
            path: resolve_path_or_none( params.first() )
        })
        },

        "" => BuiltCommand::Empty(EmptyStruct {}),

        "test" => BuiltCommand::Test(TestStruct {}),

        _ => BuiltCommand::Unrecognized(UnrecognisedStruct{cmd: cmd.to_string()}),
    }
}
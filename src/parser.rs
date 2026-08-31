use std::path::PathBuf;
use crate::command_impl::command_builder::BuiltCommand;
use crate::command_impl::file_sys::{CdStruct, LsStruct, MkDirStruct, PwdStruct, RmStruct, TouchStruct};
use crate::command_impl::general_use::{ClearStruct, EmptyStruct, ExitStruct, TestStruct, UnrecognisedStruct};
use crate::helper::{resolve_args_and_path, resolve_args_and_path_list, resolve_path_or_none};

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

        "rm" => {
            let resolved_args = resolve_args_and_path_list(&params);
            let possible_args: Option<String>= resolved_args.0;
            let possible_path: Option<Vec<String>> = resolved_args.1;

            let mut temp_path_list: Vec<Option<PathBuf>> = Vec::new();
            let mut resolved_path_vec: Option<Vec<PathBuf>> = Some(Vec::new());

            match possible_path {
                Some(p) => {
                    for path in p {
                        temp_path_list.push(resolve_path_or_none( Some(&path)))
                    }
                },
                None => {resolved_path_vec = None}
            }

            for p in temp_path_list.into_iter().flatten() {
                resolved_path_vec.as_mut().unwrap().push(p)
            }

           BuiltCommand::Rm(RmStruct{
               args: possible_args,
               path: resolved_path_vec
           })
        }

        "" => BuiltCommand::Empty(EmptyStruct {}),

        "test" => BuiltCommand::Test(TestStruct {}),

        _ => BuiltCommand::Unrecognized(UnrecognisedStruct{cmd: cmd.to_string()}),
    }
}
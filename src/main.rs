#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;
use std::{fs::DirEntry, os::unix::fs::PermissionsExt};

use anyhow::anyhow;

use crate::cmd::{Cmd, Command};

pub mod cmd;

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn parse_input(input: &str, executable_entries: &Vec<DirEntry>) -> anyhow::Result<Cmd> {
    let parts = input.split(' ').collect::<Vec<&str>>();
    if parts.len() < 1 {
        return Err(anyhow!("invalid input"));
    }
    let cmd_name = parts[0];
    let args: Vec<String> = parts[1..]
        .to_owned()
        .iter()
        .map(|s| s.to_owned().to_owned())
        .collect();

    let exec_entry = executable_entries
        .iter()
        .find(|ele| ele.file_name().to_str() == Some(cmd_name));

    match cmd_name {
        "exit" => {
            if parts.len() == 1 {
                Ok(Cmd::Exit(0))
            } else {
                let exit_code = parts[1].parse::<u8>()?;
                Ok(Cmd::Exit(exit_code))
            }
        }
        "echo" => Ok(Cmd::Echo(Command::new(cmd_name, None, args))),
        "type" => Ok(Cmd::Type(Command::new(cmd_name, None, args))),
        cmd_name if exec_entry.is_some() => {
            let exec_entry = exec_entry.unwrap();
            let exec_path = exec_entry.path();
            let path_str = exec_path
                .into_os_string()
                .into_string()
                .unwrap_or("path to string error".to_owned());
            let command = Command::new(cmd_name, Some(path_str), args);

            Ok(Cmd::Exec(command))
        }
        _ => Ok(Cmd::Unknown(Command::new(cmd_name, None, args))),
    }
}

fn get_executable_from_path_env() -> anyhow::Result<Vec<DirEntry>> {
    let path = std::env::var("PATH")?;
    let executable_paths = path.split(":").map(|path| PathBuf::from(path));
    Ok(executable_paths
        .map(std::fs::read_dir)
        .flatten()
        .flatten()
        .flatten()
        .collect::<Vec<_>>())
}

fn main() -> anyhow::Result<std::process::ExitCode> {
    let builtin_cmds = vec!["echo", "exit", "type"];
    let executable_entries = get_executable_from_path_env()?;

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        trim_newline(&mut input);

        // If input is just space
        if !input.chars().any(|c| c != ' ') {
            continue;
        }

        let cmd = parse_input(&input, &executable_entries)?;
        match cmd {
            Cmd::Echo(cmd) => {
                let echo_args = cmd.args;
                println!("{}", echo_args.join(" "))
            }
            Cmd::Exit(exit_code) => {
                return Ok(std::process::ExitCode::from(exit_code));
            }
            Cmd::Type(cmd) => {
                let type_args = cmd.args;
                for type_arg in type_args {
                    if builtin_cmds.contains(&&*type_arg) {
                        println!("{} is a shell builtin", type_arg);
                        continue;
                    }

                    if let Some(executable_entry) = executable_entries.iter().find(|entry| {
                        if let Ok(metadata) = entry.metadata() {
                            metadata.permissions().mode() & 0o111 != 0
                                && entry.file_name().to_str() == Some(type_arg.as_str())
                        } else {
                            false
                        }
                    }) {
                        println!("{} is {}", type_arg, executable_entry.path().display())
                    } else {
                        println!("{}: not found", type_arg)
                    }
                }
            }
            Cmd::Unknown(cmd) => {
                println!("{}: command not found", cmd.name)
            }
            Cmd::Exec(cmd) => {
                cmd.run()?;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() -> anyhow::Result<()> {
        let executable_entries = get_executable_from_path_env()?;
        assert_eq!(parse_input("exit 0", &executable_entries)?, Cmd::Exit(0));
        assert_eq!(
            parse_input("unknown_cmd arg_0", &executable_entries)?,
            Cmd::Unknown(Command::new("unknown_cmd", None, vec!["arg_0".to_owned()]))
        );
        assert_eq!(
            parse_input("unknown_cmd", &executable_entries)?,
            Cmd::Unknown(Command::new("unknown_cmd", None, vec![]))
        );

        Ok(())
    }

    #[test]
    fn test_path_env() -> anyhow::Result<()> {
        let executable_entries = get_executable_from_path_env()?;

        let log_str: Vec<_> = executable_entries
            .iter()
            .map(|entry| (entry.path(), entry.file_name()))
            .collect();
        println!("{:#?}", log_str);
        Ok(())
    }
}

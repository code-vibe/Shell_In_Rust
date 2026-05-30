#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;
use pathsearch::find_executable_in_path;
use std::process::Command;
use std::env;

fn main() {
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let words = shell_words::split(&input).unwrap();

        let t = words[1..].to_vec();
        let tail = words[1..].join(" ");
        match words[0].as_str() {
            "cd" => {
                let home = env::home_dir().unwrap_or("???".into()).into_os_string().into_string().unwrap();
                let subbed_tail = tail.replace("~", &home);
                if !env::set_current_dir(subbed_tail).is_ok() {
                    println!("cd: {}: No such file or directory", tail.trim());
                }
            },
            "exit" => process::exit(0),
            "echo" => println!("{}", words[1..].join(" ")),
            "pwd" => println!("{}", env::current_dir().unwrap().display()),
            "type" => {
                let trimmed = tail.trim();
                match trimmed {
                    "exit" | "echo" | "pwd" | "type" =>
                        println!("{} is a shell builtin", trimmed),
                    _ => {
                        if let Some(found) = find_executable_in_path(trimmed) {
                            println!("{} is {}", trimmed, found.display());
                        } else {
                            println!("{}: not found", trimmed);
                        }
                    },
                }
            },
            h => {
                if let Some(_found) = find_executable_in_path(h) {
                    Command::new(h).args(t).status().expect("command failed");
                } else {
                    println!("{}: command not found", h);
                }
            },
        }
    }
}
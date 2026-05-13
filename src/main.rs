#[allow(unused_imports)]
use std::io::{self, Write};

mod shell;


struct  BuiltIn {
     echo : String,
     exit : String
}
fn main() {
     loop {
          print!("$ ");
          io::stdout().flush().unwrap();
          // Wait for user input
          let mut command = String::new();
          io::stdin().read_line(&mut command).unwrap();
          command = command.trim().to_string();
          if command == "exit" {
               break;
          } else if command.starts_with("echo ") {
               println!("{}", &command[5..]);
          } else if command.starts_with("type ") {
               let output =  &command[5..];
               match output {
                    "echo" => println!("{} is a shell builtin", output.to_string()),
                    "exit" => println!("{} is a shell builtin", output.to_string()),
                    "type" => println!("{} is a shell builtin", output.to_string()),
                    _ => println!("{}: not found", output.to_string())
               }
          }else {
               println!("{}: command not found", command);
          }
     }
}

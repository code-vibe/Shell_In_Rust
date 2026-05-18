#[allow(unused_imports)]
use std::io::{self, Write};
use pathsearch::find_executable_in_path;

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

          let mut input = "".to_string();
          io::stdin().read_line(&mut input).unwrap(); //"type grep"

         let input =  input.trim();

          let command: Vec<&str> = input.split(' ').collect();

          if command[0] == "exit" {
               break;
          }
          eval_command(command[0], command[1..command.len()].to_vec());


     }
}


fn eval_command(command: &str, args: Vec<&str>)  {
     let known_commands = ["exit", "echo", "type"];

     if !known_commands.contains(&command) {
         println!("{}: command not found", command);
          return;
     }

     if command == "echo" {
          for arg in args {
               println!("{}", arg);
          }
          println!();
          return;
     }

     if command == "type" {
          if known_commands.contains(&args[0]) {
               println!("{} is a shell builtin", args[0]);
          }else if let Some(path) = find_executable_in_path(&args[0]) {
               println!("{} is {}", args[0], path.display());
          }else {
               println!("{}: not found", args[0]);
          }
     }

}

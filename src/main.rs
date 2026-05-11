#[allow(unused_imports)]
use std::io::{self, Write};

mod shell;
fn main() {

    // TODO: Uncomment the code below to pass the first stage
     print!("$ ");
     io::stdout().flush().unwrap();
     // Wait for user input
     let mut command = String::new();
     io::stdin().read_line(&mut command).unwrap();

     println!("{}", shell::handle_command(&command.as_str()));

}

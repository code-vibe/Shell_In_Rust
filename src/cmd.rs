use anyhow::{Context, anyhow};
use strum::{Display, EnumIs, EnumTryAs};

#[derive(Debug, PartialEq, EnumIs, EnumTryAs, Display)]
pub enum Cmd {
    Echo(Command),
    Exit(u8),
    Type(Command),
    Exec(Command),
    Unknown(Command),
}

impl Cmd {
    pub fn try_as_command(&self) -> anyhow::Result<Command> {
        match self {
            Cmd::Exit(_) => Err(anyhow!("exit cmd")),
            Cmd::Echo(cmd) => Ok(cmd.clone()),
            Cmd::Type(cmd) => Ok(cmd.clone()),
            Cmd::Exec(cmd) => Ok(cmd.clone()),
            Cmd::Unknown(cmd) => Ok(cmd.clone()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    // command alias name
    pub name: String,
    // actual command path
    pub path: Option<String>,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(name: &str, path: Option<String>, args: Vec<String>) -> Self {
        Self {
            name: name.to_owned(),
            path: path,
            args,
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let program = &self.name;
        let mut child = std::process::Command::new(program)
            .args(&self.args)
            .spawn()?;
        child.wait()?;

        Ok(())
    }
}
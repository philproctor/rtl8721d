use crate::alloc::string::ToString;
use crate::prelude::*;

pub enum Command {
    System(SystemCommand),
    Config(ConfigCommand),
}

impl Command {
    pub fn parse(cmd: &str) -> Result<Self> {
        let split = cmd.split_once(" ");
        if let Some((cmd_group, cmd_opts)) = split {
            let cmd_group = cmd_group.to_lowercase();
            match cmd_group.as_str() {
                "system" => Ok(Self::System(SystemCommand::parse(cmd_opts)?)),
                "config" => Ok(Self::Config(ConfigCommand::parse(cmd_opts)?)),
                s => Err(SystemError::ParseError(format!(
                    "Unknown command '{}'",
                    s.to_string(),
                ))),
            }
        } else {
            Err(SystemError::ParseError("Could not parse command!".into()))
        }
    }
}

pub enum SystemCommand {
    Reset,
    Tasks,
    Status,
}

impl SystemCommand {
    fn parse(args: &str) -> Result<Self> {
        let args = args.to_lowercase();
        match args.as_str() {
            "reset" => Ok(Self::Reset),
            "tasks" => Ok(Self::Tasks),
            "status" => Ok(Self::Status),
            s => Err(SystemError::ParseError(format!(
                "Unknown system command '{}'",
                s
            ))),
        }
    }
}

pub enum ConfigCommand {
    Get(String),
    Set(String, String),
}

impl ConfigCommand {
    fn parse(args: &str) -> Result<Self> {
        let args: Vec<&str> = args.splitn(3, " ").collect();
        if args.len() >= 2 {
            match args[0].to_lowercase().as_str() {
                "get" => Ok(ConfigCommand::Get(args[1].to_string())),
                "set" => {
                    if args.len() >= 3 {
                        Ok(ConfigCommand::Set(args[1].to_string(), args[2].to_string()))
                    } else {
                        Err(SystemError::ParseError(
                            "Set requires both config key and value".into(),
                        ))
                    }
                }
                s => Err(SystemError::ParseError(format!(
                    "Unknown config argument '{}'",
                    s
                ))),
            }
        } else {
            Err(SystemError::ParseError("Not enough arguments".into()))
        }
    }
}

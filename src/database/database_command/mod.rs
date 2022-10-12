use std::str::FromStr;
use command_type::CommandType;

pub mod command_type;

pub struct DatabaseCommand {
    pub command_type: CommandType,
    pub key: String,
    pub value: Option<String>,
}

impl DatabaseCommand {
    pub fn from(commands: Vec<&str>) -> Result<DatabaseCommand, &'static str> {
        if commands.len() < 2 {
            return Err("Command size is not correct");
        }
        let command_type = CommandType::from_str(commands[0])?;
        let key: String;
        let value: Option<String>;
        match command_type {
            CommandType::Get => {
                if commands.len() != 2 {
                    return Err("Command size is not correct");
                }
                key = commands[1].to_string();
                value = None;
            }
            CommandType::Set => {
                if commands.len() != 3 {
                    return Err("Command size is not correct");
                }
                key = commands[1].to_string();
                value = Some(commands[2].to_string());
            }
        }
        Ok(DatabaseCommand {
            command_type,
            key,
            value,
        })
    }
}

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
        let command_type = CommandType::from_str(commands[0])?;
        let key: String;
        let mut value: Option<String>;
        match command_type {
            CommandType::GET => {
                if commands.len() != 2 {
                    return Err("Command size is not correct");
                }
                key = commands[1].to_string();
                value = None;
            }
            CommandType::SET => {
                if commands.len() != 3 {
                    return Err("Command size is not correct");
                }
                key = commands[1].to_string();
                value = Some(commands[2].to_string());
            }
        }
        return Ok(DatabaseCommand {
            command_type,
            key,
            value,
        });
    }
}

use std::str::FromStr;

pub enum CommandType {
    GET,
    SET,
}

impl FromStr for CommandType {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<CommandType, Self::Err> {
        match &input.to_uppercase()[..] {
            "GET" => Ok(CommandType::GET),
            "SET" => Ok(CommandType::SET),
            _ => Err("Command is not Defined"),
        }
    }
}
use std::str::FromStr;

pub enum CommandType {
    Get,
    Set,
    Delete
}

impl FromStr for CommandType {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<CommandType, Self::Err> {
        match &input.to_uppercase()[..] {
            "GET" => Ok(CommandType::Get),
            "SET" => Ok(CommandType::Set),
            "DELETE" => Ok(CommandType::Delete),
            _ => Err("Command is not Defined"),
        }
    }
}
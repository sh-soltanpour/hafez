use std::str::FromStr;

pub enum CommandType {
    Get,
    Set,
}

impl FromStr for CommandType {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<CommandType, Self::Err> {
        match &input.to_uppercase()[..] {
            "GET" => Ok(CommandType::Get),
            "SET" => Ok(CommandType::Set),
            _ => Err("Command is not Defined"),
        }
    }
}
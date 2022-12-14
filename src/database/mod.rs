pub mod database_command;

use std::collections::HashMap;
use crate::database::database_command::DatabaseCommand;
use crate::database::database_command::command_type::CommandType;
use lazy_static::lazy_static;
use std::sync::Mutex;

pub struct Database {
    map_name: HashMap<String, String>,
}

lazy_static! {
    static ref DATABASE: Mutex<Database> = Mutex::new(Database{map_name: HashMap::new()});
}

impl Database {
    pub fn set(key: String, value: String) {
        DATABASE.lock().unwrap().map_name.insert(key, value);
    }

    pub fn get(key: String) -> Option<String> {
        let db = DATABASE.lock().unwrap();
        let value = db.map_name.get(&key)?;
        Some(String::from(value))
    }

    pub fn delete(key: String) -> Option<String> {
        DATABASE.lock().unwrap().map_name.remove(&key)
    }

    pub fn exec_command(command: DatabaseCommand) -> String {
        match command.command_type {
            CommandType::Set => {
                Self::set(command.key, command.value.unwrap());
                String::from("Key set!")
            }
            CommandType::Get => {
                Self::get(command.key).unwrap_or(String::from("Key not found!"))
            }
            CommandType::Delete => {
                match Self::delete(command.key) {
                    Some(_) => String::from("Key deleted!"),
                    None => String::from("Key Does not exist")
                }
            }
        }
    }
}

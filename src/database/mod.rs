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

    pub fn exec_command(command: DatabaseCommand) -> Option<String> {
        match command.command_type {
            CommandType::Set => {
                Self::set(command.key, command.value.unwrap());
                None
            }
            CommandType::Get => {
                Self::get(command.key)
            }
        }
    }
}

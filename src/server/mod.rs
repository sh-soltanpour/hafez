use std::fmt::Error;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use crate::database::Database;
use crate::database::database_command::DatabaseCommand;
use std::panic::resume_unwind;

pub fn listen() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
    loop {
        let mut buf = String::new();
        buf_reader.read_line(&mut buf);
        if buf.trim() == "EXIT" {
            return;
        }

        match DatabaseCommand::from(buf.split_whitespace().collect()) {
            Err(str) => {
                stream.try_clone().unwrap().write((str.to_owned() + "\n").as_ref()).unwrap();
            }
            Ok(command) => {
                if let Some(response) = Database::exec_command(command) {
                    stream.try_clone().unwrap().write((response + "\n").as_ref());
                }
            }
        }
    }
}
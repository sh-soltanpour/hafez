use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use crate::database::Database;
use crate::database::database_command::DatabaseCommand;
use threadpool::ThreadPool;

pub fn listen() {
    let pool = ThreadPool::new(30);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(stream: TcpStream) {
    let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
    loop {
        let mut buf = String::new();
        buf_reader.read_line(&mut buf).unwrap();
        if buf.trim() == "EXIT" {
            return;
        }

        match DatabaseCommand::from(buf.split_whitespace().collect()) {
            Err(str) => {
                stream.try_clone().unwrap().write_all((str.to_owned() + "\n").as_ref()).unwrap();
            }
            Ok(command) => {
                if let Some(response) = Database::exec_command(command) {
                    stream.try_clone().unwrap().write_all((response + "\n").as_ref()).unwrap();
                }
            }
        }
    }
}
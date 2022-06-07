use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use crate::database;

pub fn handle_connection(mut stream: TcpStream) {
    database::add_random_person_to_db().expect("can add new random person");
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn start_listener(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    thread::spawn(move || {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream);
        }
    });
}

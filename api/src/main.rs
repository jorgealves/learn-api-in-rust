use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result|result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request)
}

fn main() {
    let listener = TcpListener::bind("localhost:8000").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream)
    }
}

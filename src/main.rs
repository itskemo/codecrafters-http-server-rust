use std::{net::TcpListener, io::{Write, Read}};

const ADDR: &str = "127.0.0.1:4221";

fn main() {
    let listener = TcpListener::bind(&ADDR).unwrap();
    println!("Server is running at {}", ADDR);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection {}", &stream.peer_addr().unwrap());

                let mut buffer = [0; 256];

                stream.read(&mut buffer).unwrap();

                // println!("received data {:?}", buffer);

                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                stream.flush().unwrap()

            }
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }
}

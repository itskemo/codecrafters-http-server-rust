use std::{
    io::{Read, Write},
    net::TcpListener,
};

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

                let request_str = std::str::from_utf8(&buffer).unwrap();
                let lines: Vec<String> = request_str.lines().map(|line| line.to_string()).collect();
                let request_line = lines.first().unwrap().to_string();
                let chunks: Vec<&str> = request_line.split_whitespace().collect();

                println!("parsed path {:?}", chunks[1]);

                if chunks[1] == "/" {
                    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                } else if chunks[1].contains("/echo/") {
                    let content = chunks[1].replace("/echo/", "");
                    let result = format!("{}{}{}{}", "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: ", content.len(), "\r\n\r\n", content);
                    stream.write_all(result.as_bytes()).unwrap();
                } else if chunks[1] == "/user-agent" {
                    let headers_line: Vec<&String> = lines.iter().filter(|line| line.contains("User-Agent")).collect();
                    // println!("{:?}", headers_line);
                    let content = headers_line[0].replace("User-Agent: ", "");
                    let result = format!("{}{}{}{}", "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: ", content.len(), "\r\n\r\n", content);
                    stream.write_all(result.as_bytes()).unwrap();
                } else {
                    stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
                }

                stream.flush().unwrap()
            }
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }
}

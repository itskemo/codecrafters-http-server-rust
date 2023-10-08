use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream}, collections::HashMap,
};
use std::thread;

const ADDR: &str = "127.0.0.1:4221";

fn parse_request(stream: &mut TcpStream) -> String {
    let mut buffer: [u8;512] = [0;512];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    request.to_string()
}

fn get_path(request: &str) -> Vec<&str> {
    let lines = request.split("\r\n");
    let mut path: Vec<&str> = Vec::new();
    for line in lines {
        if line.starts_with("GET") {
            let mut path_string = line.split(" ").nth(1).unwrap();
            path_string = path_string.trim();

            for subs in path_string.split("/") {
                path.push(subs);
            }
        }
    }
    path
}

fn get_header(request: &str) -> HashMap<String, String> {
    let mut header: HashMap<String, String> = HashMap::new();
    let lines = request.split("\r\n");

    for line in lines {
        if !line.starts_with("GET") {
            let key_value: Vec<&str> = line.split(": ").collect();

            if key_value.len() > 1 {
                header.insert(key_value.get(0).unwrap().to_string(), key_value.get(1).unwrap().to_string());
            }
        }
    }
    header
}


fn handle_client(mut stream: TcpStream) {
    let request = parse_request(&mut stream);
    let path = get_path(&request);
    let header = get_header(&request);

    let response = match path[1] {
        "" => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
        "echo" => {
            let echo_path: Vec<&str> = path[2..].to_vec();
            let mut echo = echo_path[0].to_string();

            for sub in echo_path[1..].iter() {
                echo += "/";
                echo += sub;
            }

            format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", echo.len(), echo)
        },
        "user-agent" => {
            let agent = &header["User-Agent"];
            format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", agent.len(), agent)
        },
        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind(&ADDR).unwrap();
    println!("Server is running at {}", ADDR);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                thread::spawn(move || {
                  handle_client(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

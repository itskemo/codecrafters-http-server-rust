use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream}, collections::HashMap,
};
use std::thread;
use std::env;
use std::fs::read_to_string;
use std::fs::File;


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
        if line.starts_with("GET") || line.starts_with("POST") {
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
        if !line.starts_with("GET") || !line.starts_with("POST") {
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
    let args: Vec<String> = env::args().collect();

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
        "files" => {
            let directory = if &args[2].clone() == "/" || &args[2].clone() == "" {
                env::current_dir().unwrap()
            } else {
                env::current_dir().unwrap().join(&args[2].clone())
            };

            let file_path = directory.join(path[2]);

            println!("working file {:?}", &file_path);

            let method: Vec<&str> = request.split(" ").collect();

            if method[0] == "GET" {
                if std::path::Path::new(&file_path).exists() {
                    let file = read_to_string(&file_path).unwrap();

                    format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",file.len(),file)
                } else {
                    "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
                }
            } else if method[0] == "POST" {
                let file_content: Vec<&str> = request.split("\r\n\r\n").collect();
                let mut file = File::create(&file_path).expect("could not create file");

                file.write_all(file_content[1].as_bytes()).expect("could not write to file");

                "HTTP/1.1 201 Created\r\n\r\n".to_string()
            } else {
                "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
            }

        },
        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
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

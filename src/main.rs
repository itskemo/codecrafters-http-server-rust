use std::net::TcpListener;

const ADDR: &str = "127.0.0.1:4221";

fn main() {
    let listener = TcpListener::bind(&ADDR).unwrap();
    println!("Server is running at {}", ADDR);

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection")
            }
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(_stream) => {
    //             println!("accepted new connection");
    //         }
    //         Err(e) => {
    //             println!("error: {}", e);
    //         }
    //     }
    // }
}

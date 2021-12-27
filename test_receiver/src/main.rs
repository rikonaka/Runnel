use threadpool::ThreadPool;
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, Read};

struct GlobalConfig {
    listen_address: String,
    port: i32,
    token: String,
}

impl GlobalConfig {
    fn new() -> GlobalConfig {
        let global_config = GlobalConfig {
            listen_address: String::from("127.0.0.1"),
            port: 11080,
            token: String::from("opopop"),
        };
        return global_config;
    }
}

fn handle_connection(stream: TcpStream) {

    // let mut sender = BufWriter::new(&stream);
    let mut reader = BufReader::new(&stream);

    let mut buff = [0 as u8; 1024];
    let read_size = reader.read(&mut buff).unwrap();
    println!("read_size: {}", read_size);
    println!("read_contents: {}", String::from_utf8_lossy(&buff));

    // let mut data = [0 as u8; 512];
    // stream.read(&mut data).unwrap();
    /*
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
        },
    }
    */
}

fn server(address: String, port: i32, token: String) {
    // let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    println!("token - {}", token);
    let mut bind_address_with_port: String = String::new();
    bind_address_with_port += &address;
    bind_address_with_port += ":";
    bind_address_with_port += &port.to_string();
    let listener = TcpListener::bind(bind_address_with_port).unwrap();
    // listener.set_nonblocking(true).expect("Cannot set non-blocking");

    let workers = 4;
    let pool = ThreadPool::new(workers);

    for stream in listener.incoming() {
        // println!("Connection established!");
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => println!("Failed to get stream: {}", e),
        }
    }
}

fn main() {
    let global_config = GlobalConfig::new();
    server(global_config.listen_address, global_config.port, global_config.token);
}
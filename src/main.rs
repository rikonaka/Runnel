use clap::Parser;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

/// Runnel tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Server or client.
    #[clap(
        short,
        long,
        value_parser,
        default_value = "client",
        default_missing_value = "yes"
    )]
    style: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 3333)]
    port: u32,

    /// Listen address
    #[clap(short, long, value_parser, default_value = "127.0.0.1")]
    address: String,
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let args = Args::parse();
    let bind_string = format!("{}:{}", args.address, args.port);
    match args.style.as_str() {
        "server" => {
            let listener = TcpListener::bind(bind_string).unwrap();
            // accept connections and process them, spawning a new thread for each one
            println!("Server listening on port 3333");
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("New connection: {}", stream.peer_addr().unwrap());
                        thread::spawn(move || {
                            // connection succeeded
                            handle_client(stream)
                        });
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        /* connection failed */
                    }
                }
            }
            // close the socket server
            drop(listener);
        }
        "client" => {
            // client code here
        }
        _ => (),
    }
}

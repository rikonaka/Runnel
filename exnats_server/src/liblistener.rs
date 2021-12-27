use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use threadpool::ThreadPool;
use hex;

// my crypto crate
use crypto;
use error::{AuthError, ReadSPError};

fn read_specify_size(reader: &mut BufReader<&TcpStream>, size: usize) -> Result<Vec<u8>, ReadSPError> {
    let mut buff = vec![0; size];
    match reader.read(&mut buff) {
        Ok(read_size) => {
            if read_size != size {
                read_specify_size(reader, size - read_size);
            }
        }
        Err(e) => {
            println!("read failed {}", e);
            return Err(ReadSPError);
        }
    }
    return Ok(buff);
}

fn connection_authentication(sender: &mut BufWriter<&TcpStream>, reader: &mut BufReader<&TcpStream>) -> Result<Vec<u8>, AuthError> {
    // 1 - packet send from client ask server provide the rsa public key
    // public key || hash(public key) || sign(public key);
    let mut hello_buff = [0 as u8; 5];
    match reader.read(&mut hello_buff) {
        // Ok(read_size) => println!("read {} bytes data", read_size),
        Ok(_) => (),
        Err(e) => {
            println!("read error {}", e);
            return Err(AuthError);
        },
    }
    println!("read_contents: {}", String::from_utf8_lossy(&hello_buff));
    // println!("{:?}", String::from_utf8_lossy(&buff));
    // println!("1 - {}", hex::encode(&buff));
    // sender.write(&buff);
    // sender.flush().unwrap();
    let passphrase = crypto::random_bit_gen(64 as u32);
    let (private_key, public_key) = crypto::rsa_key_gen_openssl(&passphrase);
    let mut auth_packet: Vec<u8> = Vec::new();
    auth_packet.extend(&public_key);
    let hash = crypto::sha256_openssl(&public_key);
    auth_packet.extend(&hash);
    let sign = crypto::rsa_private_key_sign_openssl(&private_key, &passphrase, &hash);
    auth_packet.extend(&sign);

    // println!("{}", public_key.len()); // 182
    // println!("hash len {}", hash.len()); // 32
    // println!("sign len {}", sign.len()); // 64

    match sender.write(&auth_packet) {
        // Ok(send_size) => println!("we send {} bytes data", send_size),
        Ok(_) => (),
        Err(e) => {
            println!("send error {}", e);
            return Err(AuthError);
        },
    }
    sender.flush().unwrap();
    let mut session_packet_buff = [0 as u8; 64];
    match reader.read(&mut session_packet_buff) {
        Ok(read_size) => {
            if read_size != 64 {
                println!("read {} bytes session buff", read_size);
                reader.read(&mut session_packet_buff).unwrap();
            }
        },
        Err(e) => {
            println!("read error {}", e);
            return Err(AuthError);
        },
    }

    let session_packet_40 = crypto::rsa_private_key_decrypt_openssl(&private_key, &passphrase, &session_packet_buff.to_vec());
    return Ok(session_packet_40);
}

fn handle_connection(stream: TcpStream, tx: Sender<&[u8]>) {

    let mut sender = BufWriter::new(&stream);
    let mut reader = BufReader::new(&stream);
    let session_packet = match connection_authentication(&mut sender, &mut reader) {
        Ok(session_packet) => session_packet,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };
    let session_key_len = 32;
    let session_nonce_len = 8;
    let session_key_i = session_key_len;
    let sesion_nonce_i = session_key_i + session_nonce_len;
    let session_key = session_packet[0..session_key_i].to_vec();
    let session_nonce = session_packet[session_key_i..sesion_nonce_i].to_vec();
    println!("{}", hex::encode(&session_key));
    println!("{}", hex::encode(&session_nonce));

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

pub fn server(address: String, port: i32, token: String) {
    // let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    let mut bind_address_with_port: String = String::new();
    bind_address_with_port += &address;
    bind_address_with_port += ":";
    bind_address_with_port += &port.to_string();
    println!("listen: {}, port: {}", address, port);

    let listener = TcpListener::bind(bind_address_with_port).unwrap();
    // listener.set_nonblocking(true).expect("Cannot set non-blocking");

    let workers = 4;
    let pool = ThreadPool::new(workers);

    let (tx, tc): (Sender<&[u8]>, Receiver<&[u8]>) = channel();
    for stream in listener.incoming() {
        // println!("Connection established!");
        let tx = tx.clone();
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream, tx);
                });
            }
            Err(e) => println!("Failed to get stream: {}", e),
        }
    }
}

use std::net::{TcpStream, Shutdown};
// use std::io::{Read, Write};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::sync::mpsc::{channel, Receiver, Sender};
// use std::process;
use threadpool::ThreadPool;
use hex;

use crypto;
use error::AuthError;

fn connection_authentication(sender: &mut BufWriter<&TcpStream>, reader: &mut BufReader<&TcpStream>) -> Result<Vec<u8>, AuthError> {
    // 1 - send hello to server
    let hello = b"hello";
    match sender.write(hello) {
        Ok(send_size) => println!("we send {} bytes hello", send_size),
        Err(e) => println!("send error {}", e),
    }
    sender.flush().unwrap();
    // 2 - read the public key from server and verify
    let mut read_buff = [0 as u8; 278];
    match reader.read(&mut read_buff) {
        Ok(read_size) => {
            if read_size != 278 {
                println!("read {} bytes data", read_size);
                reader.read(&mut read_buff).unwrap();
            }
        },
        Err(e) => println!("read error {}", e),
    }

    let public_key_len = 182; // pem style
    let public_key_hash_len = 32; 
    let public_key_sign_len = 64;
    let public_key_i = public_key_len;
    let public_key_hash_i = public_key_i + public_key_hash_len;
    let public_key_sign_i = public_key_hash_i + public_key_sign_len;

    let public_key_182: Vec<u8> = read_buff[0..public_key_i].to_vec();
    let hash_32: Vec<u8> = read_buff[public_key_i..public_key_hash_i].to_vec();
    let sign_64: Vec<u8> = read_buff[public_key_hash_i..public_key_sign_i].to_vec();

    // first check public key's hash
    let public_key_182_hash = crypto::sha256_openssl(&public_key_182);
    if public_key_182_hash != hash_32 {
        return Err(AuthError);
    }
    let verify_32 = crypto::rsa_public_key_verify_openssl(&public_key_182, &sign_64);
    // println!("{}", verify_32.len());
    if verify_32 != hash_32 {
        return Err(AuthError);
    }

    let session_key = crypto::random_bit_gen(32 as u32);
    let session_nonce = crypto::random_bit_gen(8 as u32);
    let mut session_packet: Vec<u8> = Vec::new();
    session_packet.extend(session_key);
    session_packet.extend(session_nonce);
    let encrypt_session_packet = crypto::rsa_public_key_encrypt_openssl(&public_key_182, &session_packet);
    match sender.write(&encrypt_session_packet) {
        Ok(send_size) => println!("we send {} bytes", send_size), // 64
        Err(e) => println!("send error {}", e),
    }
    sender.flush().unwrap();
    return Ok(session_packet);
}

fn handle_connection(stream: TcpStream, tx: Sender<&[u8]>) {
    let mut sender = BufWriter::new(&stream);
    let mut reader = BufReader::new(&stream);
    let session_packet = match connection_authentication(&mut sender, &mut reader) {
        Ok(session_packet) => session_packet,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let session_key_len = 32;
    let session_nonce_len = 8;
    let session_key_i = session_key_len;
    let sesion_nonce_i = session_key_i + session_nonce_len;
    let session_key = session_packet[0..session_key_i].to_vec();
    let session_nonce = session_packet[session_key_i..sesion_nonce_i].to_vec();
    println!("{}", hex::encode(&session_key));
    println!("{}", hex::encode(&session_nonce));
    stream.shutdown(Shutdown::Both).expect("failed to shutdown");
}

pub fn client(server_address: &String, server_port: &i32) {
    let mut server = String::new();
    server += server_address;
    server += ":";
    server += &server_port.to_string();

    let workers = 4;
    let pool = ThreadPool::new(workers);

    let (tx, tc): (Sender<&[u8]>, Receiver<&[u8]>) = channel();
    let tx = tx.clone();
    match TcpStream::connect(server) {
        Ok(stream) => {
            println!("Successfully connected to server {} in port {}", server_address, server_port);
            // let msg = b"Hello!";
            println!("Connection to server: {:?}:{:?}!", server_address, server_port);
            pool.execute(|| {handle_connection(stream, tx);});
            pool.join();
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        },
    }
    println!("Client terminated!");
}
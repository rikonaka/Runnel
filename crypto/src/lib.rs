use std::time::SystemTime;

use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;
use openssl::hash::{hash, MessageDigest};

use rand::Rng;

use salsa20::{Salsa20, Key, Nonce};
use salsa20::cipher::{NewCipher, StreamCipher, StreamCipherSeek};
use salsa20::cipher::errors::LoopError;
// use std::fmt;
// use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme, pkcs1::ToRsaPublicKey};
// use sha2::{Sha256, Digest};


#[cfg(test)]
mod tests {
    #[test]
    fn test_gen_random_bit() {
        let bit_length = 128;
        let _: Vec<u8> = super::random_bit_gen(bit_length);
        // println!("{:?}", result);
    }
    #[test]
    fn test_rsa_openssl() {
        let start_time = super::_get_start_time();
        let data = "1234567890".as_bytes().to_vec();
        let passphrase = "rust_test".as_bytes().to_vec();
        let (private_key, public_key) = super::rsa_key_gen_openssl(&passphrase);
        let encrypted = super::rsa_public_key_encrypt_openssl(&public_key, &data);
        let data_1 = super::rsa_private_key_decrypt_openssl(&private_key, &passphrase, &encrypted);
        super::_print_exec_time(start_time, "rsa time"); // about 5.962306ms faster than rust carte 'rsa'
        let mut new_data = data.clone();
        let padding_size = data_1.len() - data.len();
        for _ in 0..padding_size {
            new_data.push(0);
        }
        assert_eq!(new_data, data_1);

    }
    #[test]
    fn test_sha256_openssl() {
        let data = "1234567890".as_bytes().to_vec();
        let result = super::sha256_openssl(&data);
        let result_hex = hex::encode(&result);
        println!("{:?}", result_hex);
        let result_should = "c775e7b757ede630cd0aa1113bd102661ab38829ca52a6422ab782862f268646";
        assert_eq!(result_hex, result_should);
    }
    #[test]
    fn test_rsa_sign_verify() {
        let data = "1234567890".as_bytes().to_vec();
        let passphrase = "rust_test".as_bytes().to_vec();
        let (private_key, public_key) = super::rsa_key_gen_openssl(&passphrase);
        let sign_data = super::rsa_private_key_sign_openssl(&private_key, &passphrase, &data);
        let verify_data = super::rsa_public_key_verify_openssl(&public_key, &sign_data);
        let mut new_data = data.clone();
        let padding_size = verify_data.len() - new_data.len();
        for _ in 0..padding_size {
            new_data.push(0);
        }
        assert_eq!(new_data, verify_data);
    }
    #[test]
    fn test_salsa20() {
        let key = super::random_bit_gen(32 as u32); // 32 bytes
        let nonce = super::random_bit_gen(8 as u32); // 8 bytes
        let some_str = String::from("this is secret text.");
        let mut data = some_str.as_bytes().to_vec();
        let origin_data = data.clone();

        let mut cipher = super::salsa20_gen_cipher(&key, &nonce);
        match super::salsa20_stream_encrypt(&mut cipher, &mut data) {
            Err(_) => assert_eq!(1, 2), 
            _ => println!("{:?}", data),
        }
        match super::salsa20_stream_decrypt(&mut cipher, 0, &mut data) {
            Err(_) => assert_eq!(1, 3),
            _ => println!("{:?}", data),
        }
        assert_eq!(data, origin_data);
    }
}

fn _get_start_time() -> SystemTime {
    return SystemTime::now();
}

fn _print_exec_time(start_time: SystemTime, some_string: &str) {
    let end_time = SystemTime::now();
    let difference = end_time.duration_since(start_time).expect("Clock may have gone backwards");
    println!("{} - {:?}", some_string, difference);
}

pub fn random_bit_gen(bit_length: u32) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..bit_length).map(|_| { rng.gen::<u8>() }).collect();
    /*
    let mut result: Vec<u8> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..bit_length {
        result.push(rng.gen::<u8>());
    }
    return result;
    */
    return random_bytes;
}

pub fn rsa_key_gen_openssl(passphrase: &Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let key_size = 512;
    let rsa = Rsa::generate(key_size).unwrap();
    // let private_key = rsa.private_key_to_pem().unwrap();
    // let public_key = rsa.public_key_to_pem_pkcs1().unwrap();
    let private_key: Vec<u8> = rsa.private_key_to_pem_passphrase(Cipher::aes_128_cbc(), &passphrase).unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();
    // let private_key_clone = private_key.clone();
    // let public_key_clone = public_key.clone();
    // println!("Private key: {}", String::from_utf8(private_key_clone).unwrap());
    // println!("Public key: {}", String::from_utf8(public_key_clone).unwrap());
    return (private_key, public_key);
}

pub fn rsa_public_key_encrypt_openssl(public_key: &Vec<u8>, plaintext: &Vec<u8>) -> Vec<u8> {
    /* get the public key from internet
    so its will be the Vec<u8> type
    */
    // Encrypt with public key
    let rsa = Rsa::public_key_from_pem(public_key).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    match rsa.public_encrypt(plaintext, &mut buf, Padding::PKCS1) {
        Err(e) => println!("encrypt error {}", e),
        _ => (),
    }
    return buf;
}

pub fn rsa_private_key_decrypt_openssl(private_key: &Vec<u8>, passphrase: &Vec<u8>, ciphertext: &Vec<u8>) -> Vec<u8> {
    // Decrypt with private key
    let rsa = Rsa::private_key_from_pem_passphrase(private_key, passphrase).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    match rsa.private_decrypt(ciphertext, &mut buf, Padding::PKCS1) {
        Err(e) => println!("decrypt error {}", e),
        _ => (),
    }
    // remove all zero value
    // buf.retain(|&x| x != 0);
    return buf;
}

pub fn rsa_private_key_sign_openssl(private_key: &Vec<u8>, passphrase: &Vec<u8>, plaintext: &Vec<u8>) -> Vec<u8> {
    let rsa = Rsa::private_key_from_pem_passphrase(private_key, &passphrase).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    match rsa.private_encrypt(plaintext, &mut buf, Padding::PKCS1) {
        Err(e) => println!("sign error {}", e),
        _ => (),
    }
    return buf;
}

pub fn rsa_public_key_verify_openssl(public_key: &Vec<u8>, ciphertext: &Vec<u8>) -> Vec<u8> {
    let rsa = Rsa::public_key_from_pem(public_key).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    match rsa.public_decrypt(ciphertext, &mut buf, Padding::PKCS1) {
        Err(e) => println!("verify error {}", e),
        _ => (),
    }
    // buf.retain(|&x| x != 0);
    return buf;
}

pub fn sha256_openssl(data: &Vec<u8>) -> Vec<u8> {
    let res = hash(MessageDigest::sha256(), data).unwrap();
    // result.copy_from_slice(&res);
    let result = (*res).to_vec();
    // println!("sha256 len {}", result.len());
    return result;
}

pub fn salsa20_gen_cipher(key_bytes: &Vec<u8>, nonce_bytes: &Vec<u8>) -> Salsa20 {
    let key = Key::from_slice(key_bytes);
    let nonce = Nonce::from_slice(nonce_bytes);
    let cipher = Salsa20::new(&key, &nonce);
    // println!("cipher get");
    return cipher;
}

pub fn salsa20_stream_encrypt(cipher: &mut Salsa20, data: &mut Vec<u8>) -> Result<(), LoopError> {
    // let key = Key::from_slice(b"an example very very secret key."); // 32 bytes = 256 bits
    // let nonce = Nonce::from_slice(b"a nonce.");                     //  8 bytes = 64 bits
    // create cipher instance
    // let mut cipher = Salsa20::new(&key, &nonce);
    // apply keystream (encrypt)
    return cipher.try_apply_keystream(data);
}

pub fn salsa20_stream_decrypt(cipher: &mut Salsa20, pos: u32, data: &mut Vec<u8>) ->Result<(), LoopError> {
    cipher.seek(pos);
    return cipher.try_apply_keystream(data);
}
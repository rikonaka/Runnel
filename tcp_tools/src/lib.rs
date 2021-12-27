use std::net::{TcpStream, Shutdown};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use log::{info, warn, error};
use error::{TcpReadError, TcpSendError};
use hex;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

const tcp_padding: Vec<u8> = hex::decode("ffffffff").unwrap();
const tcp_read_size: usize = 65535;

pub fn tcp_send(sender: &mut BufWriter<&TcpStream>, data: &mut Vec<u8>) -> Result<usize, TcpSendError> {
    // only send
    // set the tail of the packet
    data.extend(tcp_padding);
    match sender.write(data) {
        Ok(size) => {
            if size != data.len() {
                error!("send size({}) != data size({})", size, data.len());
            }
            return Ok(size);
        },
        Err(e) => {
            error!("{}", e);
            return Err(TcpSendError);
        },
    }
}


pub fn tcp_read(reader: &mut BufReader<&TcpStream>) -> Result<Vec<u8>, TcpReadError> {
    // only read
    let mut buff = [0 as u8; tcp_read_size];
    match reader.read(&mut buff) {
        Err(e) => return Err(TcpReadError),
        _ => Ok(buff.to_vec()),
    }
}
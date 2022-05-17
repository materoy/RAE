use std::{
    fs::{self},
    io::Read,
};

use bytes::BufMut;

pub fn read_bin_file(path: &str) -> Vec<u8> {
    let mut file = fs::File::open(path).expect("Failed to open binary file");
    let mut buf = [0; 1024];
    let mut file_byte = Vec::new();
    loop {
        match file.read(&mut buf) {
            Ok(n) if n == 0 => break,
            Ok(n) => file_byte.put_slice(&buf[..n]),
            Err(e) => {
                eprintln!("Faile to read bin file: {}", e);
                break;
            }
        };
    }
    file_byte
}

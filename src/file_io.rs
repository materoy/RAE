use std::{
    fs::{self, File},
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
            Ok(n) => {
                file_byte.put_slice(&buf[..n])
            }
            Err(e) => {
                eprintln!("Faile to read bin file: {}", e);
                break;
            }
        };
    }
    file_byte
}

pub fn create_bin_file(file_name: &str) -> File {
    let file = File::create(file_name).expect("Unable to create temp file");
    // unix::fs::fchown(&file, Some(1000), Some(1000)).expect("Could not make file executable");
    crate::executor::execute_command("chmod", vec!["+x", file_name]);
    file
}

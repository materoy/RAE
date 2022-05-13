use std::{fs::{self, File}, io::Read};

use crate::executor::execute_command;

pub fn read_bin_file(path: &str) -> Vec<u8> {
    let mut file = fs::File::open(path).expect("Failed to open binary file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to read binary file");
    buf
}

pub fn create_bin_file(file_name: &str) -> File {
    let file = File::create(file_name).expect("Unable to create temp file");
    // unix::fs::fchown(&file, Some(1000), Some(1000)).expect("Could not make file executable");
    execute_command("/usr/bin/chmod", vec!["+x", file_name]);
    file
}

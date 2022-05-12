use std::fs::{self, File};
use std::os::unix;

pub fn read_bin_file(path: &str) -> Vec<u8> {
    fs::read(path).expect("Failed to read binary file")
}


pub fn create_bin_file(file_name: &str) -> File {
    let file = File::create(file_name).expect("Unable to create temp file");
    unix::fs::fchown(&file, Some(1000), Some(1000)).expect("Could not make file executable");
    file
}

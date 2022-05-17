use std::fs::File;

use super::executor;


pub fn create_bin_file(file_name: &str) -> File {
    let file = File::create(file_name).expect("Unable to create temp file");
    // unix::fs::fchown(&file, Some(1000), Some(1000)).expect("Could not make file executable");
    executor::execute_command("chmod", vec!["+x", file_name]);
    file
}

use std::fs::File;

use std::fs::OpenOptions;
use std::os::unix::prelude::OpenOptionsExt;

pub async fn create_bin_file(file_name: &str) -> File {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .mode(0o101)
        .open(file_name)
        .expect("Cannot create file");

    file
}

pub async fn delete_file_async(file_name: &str) {
    match tokio::fs::remove_file(file_name).await {
        Ok(_) => {
            println!("File deleted successfully")
        }
        Err(e) => {
            eprintln!("Error deleting file: {}", e);
        }
    }
}

pub fn delete_file_sync(file_name: &str) {
    match std::fs::remove_file(file_name) {
        Ok(_) => {
            println!("File deleted successfully")
        }
        Err(e) => {
            eprintln!("Error deleting file: {}", e);
        }
    }
}
pub fn delete_all_in_dir(dir: &str) {
    let read_dir = std::fs::read_dir(dir).expect(format!("Failed to read dir {}", dir).as_str());
    for dir_entry in read_dir  {
        let dir_entry = dir_entry.unwrap();
        let path = dir_entry.path().into_os_string().into_string().expect("Failed to get path string");
       delete_file_sync(&path); 
    }
}

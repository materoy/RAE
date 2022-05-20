use tokio::fs::File;

use tokio::fs::OpenOptions;


pub async fn create_bin_file(file_name: &str) -> File {
    
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .mode(0o111)
        .open(file_name)
        .await.expect("Cannot create file");

    file
}

pub async fn delete_file(file_name: &str) {
    match tokio::fs::remove_file(file_name).await {
        Ok(_) => { println!("File deleted successfully")},
        Err(e) => {
            eprintln!("Error deleting file: {}", e);
        }
    }
}
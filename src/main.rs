#![feature(unix_chown)]

use std::{
    env,
    fs::{self, File},
    process::Command,
};


mod client;
mod server;

/*
    This command executes a binary given its path
    then returns and prints out its's output
*/
fn execute_bin(path: &str) -> String {
    let mut command = Command::new(format!("./{}", path));

    println!("Executing {}...", command.get_program().to_str().unwrap());
    match command.output() {
        Ok(output) => {
            let output_string = String::from_utf8_lossy(&output.stdout);
            println!("Output {:?}", output_string);
            output_string.to_string()
        }
        Err(e) => {
            eprintln!("ERROR RUNNING BINRARY: {}", e);
            String::from("Some problem here your application could not be executed")
        }
    }
}

fn read_bin_file(path: &str) -> Vec<u8> {
    fs::read(path).expect("Failed to read binary file")
}

use std::os::unix;

fn create_bin_file(file_name: &str) -> File {
    let file = File::create(file_name).expect("Unable to create temp file");
    unix::fs::fchown(&file, Some(1000), Some(1000)).expect("Could not make file executable");
    file
}

#[tokio::main]
async fn main() {
    // let exec_dir = "./exec/target/release/exec";
    // execute_bin(exec_dir);

    let args: Vec<String> = env::args().collect();

    let run_as_server = match args.get(1) {
        Some(_) => false,
        None => true,
    };

    let addr = "127.0.0.1:5050";
    if run_as_server {
        server::server(addr).await.unwrap();
    } else {
        client::client(addr).await;
    }
}

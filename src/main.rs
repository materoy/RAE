use std::{env, io::{Write, Read}, net::{TcpListener, TcpStream}, process::Command};

#[allow(dead_code)]

/*
    This command executes a binary given its path
    then returns and prints out its's output
*/
fn execute_bin(path: &str) -> String {
    let mut command = Command::new(path);

    println!("Executing {}...", command.get_program().to_str().unwrap());
    let output = command
        .output()
        .map(|output| {
            let output_string = String::from_utf8_lossy(&output.stdout);
            println!("Output {:?}", output_string);
            output_string.to_string()
        })
        .unwrap();
    output
}

fn main() {
    // let exec_dir = "./exec/target/release/exec";
    // execute_bin(exec_dir);

    let args: Vec<String> = env::args().collect();

    let run_as_server = match args.get(1) {
        Some(_) => false,
        None => true,
    };

    let addr = "127.0.0.1:5050";

    if run_as_server {
        let listener = TcpListener::bind(addr).unwrap();
        print!(
            "Listeneing started on address: {} \n",
            listener.local_addr().unwrap()
        );
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            stream.write(b"Hello world\r\n").unwrap();
        }
    } else {
        if let Ok(mut stream) = TcpStream::connect(addr) {
           println!("Connected to server"); 
           let mut buf:  Vec<u8> = Vec::new(); 
           stream.read(&mut buf).map(|result| {
               println!("Read {} bytes from server", result);
           }).unwrap();
        } else {
            println!("Couldn't connect to server");
        }
    }
}

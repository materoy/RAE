use std::{
    env,
    fs::{self, File},
    io::Write,
    process::Command,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[allow(dead_code)]

/*
    This command executes a binary given its path
    then returns and prints out its's output
*/
fn execute_bin(path: &str) -> String {
    let mut command = Command::new(format!("./{}", path));

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

fn read_bin_file(path: &str) -> Vec<u8> {
    fs::read(path).expect("Failed to read binary file")
}

async fn server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Connected to client: {}", socket.peer_addr().unwrap());
        tokio::spawn(async move {
            let mut file = File::create("test_executable").expect("Unable to create temp file");
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                println!("Read {} bytes from client", n);
                if n > 0 {
                    file.write_all(&buf).expect("Unable to write to file");
                    let output = execute_bin("test_executable");

                    // Write IO output back to the socket
                    if let Err(e) = socket.write_all(output.as_bytes()).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                    }
                }
            }
        });
    }
}

async fn client(server_addr: &str) {
    if let Ok(mut socket) = TcpStream::connect(server_addr).await {
        println!("Connected to server");

        if let Err(e) = socket.write(b"Hello world").await {
            eprintln!("failed to write to socket; err = {:?}", e);
        }

        let mut buf = [0; 1024];
        loop {
            // Send binary file to server
            let path = "./exec/target/release/exec";
            if let Err(e) = socket.write(&read_bin_file(path)).await {
                eprintln!("failed to write to socket; err = {:?}", e);
            }

            println!("File sent to server.. wait for reply");

            let n = match socket.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };
            println!("Read {} bytes from server", n);

            if n > 0 {
                println!("{}", String::from_utf8_lossy(&buf))
            }
        }
    } else {
        println!("Couldn't connect to server");
    }
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
        server(addr).await.unwrap();
    } else {
        client(addr).await;
    }
}

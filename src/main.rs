use std::{env, process::Command};

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

async fn server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Connected to client: {}", socket.peer_addr().unwrap());
        tokio::spawn(async move {
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
                    println!("{}", String::from_utf8_lossy(&buf))
                }

                // Write all data back to the socket
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
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
                println!("{}", String::from_utf8_lossy(&buf))
            }

            // Write all data back to the socket
            if let Err(e) = socket.write_all(&buf[0..n]).await {
                eprintln!("failed to write to socket; err = {:?}", e);
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

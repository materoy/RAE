use std::io::Write;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::*;

pub async fn server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Connected to client: {}", socket.peer_addr().unwrap());
        tokio::spawn(async move {
            let mut file = file_io::create_bin_file("test_executable");
            let mut buf = [0;1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => break,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        break;
                    }
                };

                println!("Read {} bytes from client", n);
                match file.write_all(&buf) {
                    Ok(_) => println!("File written"),
                    Err(e) => eprintln!("Problem writing to file: {}", e),
                };
            }

            let output = executor::execute_bin("test_executable");

            // Write IO output back to the socket
            if let Err(e) = socket.write_all(output.as_bytes()).await {
                eprintln!("failed to write to socket; err = {:?}", e);
            }
        });
    }
}

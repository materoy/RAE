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
            let mut buf = bytes::BytesMut::with_capacity(1024);

            'buff_loop: loop {
                match socket.read_buf(&mut buf).await {
                    Ok(n) if n == 0 => {
                        println!("EOF reached");
                        break 'buff_loop;
                    }
                    Ok(n) => {
                        match file.write_all(&buf[0..n]) {
                            Ok(_) => {}
                            Err(e) => eprintln!("Problem writing to file: {}", e),
                        };
                    }
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        break 'buff_loop;
                    }
                };
            }

            println!("Did we ever get here ?");

            let output = executor::execute_bin("test_executable");

            // Write IO output back to the socket
            if let Err(e) = socket.write_all(output.as_bytes()).await {
                eprintln!("failed to write to socket; err = {:?}", e);
            }
        });
    }
}

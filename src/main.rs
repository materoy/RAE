#![feature(unix_chown)]

use std::env;

mod client;
mod file_io;
mod server;
mod executor;

#[tokio::main]
async fn main() {
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

use std::io::{IoSlice, Write};
use std::vec;

use application_proto::stream_service_server::{StreamService, StreamServiceServer};
use application_proto::{ApplicationRequest, ApplicationResponse, Input};
use tonic::{transport::Server, Request, Response, Status};

mod application;
mod server;

pub mod application_proto {
    tonic::include_proto!("application");
}

#[derive(Debug, Default)]
pub struct ApplicationService {}

#[tonic::async_trait]
impl StreamService for ApplicationService {
    async fn start_application(
        &self,
        reqest: Request<ApplicationRequest>,
    ) -> Result<Response<ApplicationResponse>, Status> {
        let req = reqest.into_inner();
        println!("{}", req.name);

        let file_path = format!("bin/{}", req.name);
        let mut file = server::file_io::create_bin_file(&file_path);

        file.write_vectored(&vec![IoSlice::new(req.executable.as_slice())]).expect("Failed to write to file");

        let output = server::executor::execute_bin(&file_path);

        println!("Output: {}", output);

        Ok(Response::new(ApplicationResponse {
            result: output 
        }))
    }

    async fn stream_input(
        &self,
        reqest: Request<Input>,
    ) -> Result<Response<ApplicationResponse>, Status> {
        let req = reqest.into_inner();
        println!("{}", req.input);

        Ok(Response::new(ApplicationResponse {
            result: String::from("Hi there, you happly?"),
        }))
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:5050".parse()?;

    let application_service = ApplicationService::default();

    Server::builder()
        .add_service(StreamServiceServer::new(application_service))
        .serve(addr)
        .await?;

    // let listener = TcpListener::bind(addr).await?;

    // println!("Server started at: {} ..", addr);

    // loop {
    //     let (mut socket, _) = listener.accept().await?;
    //     println!("Connected to client: {}", socket.peer_addr().unwrap());
    //     tokio::spawn(async move {
    //         let mut file = server::file_io::create_bin_file("test_executable");
    //         let mut buf = bytes::BytesMut::with_capacity(1024);

    //         'buff_loop: loop {
    //             match socket.read_buf(&mut buf).await {
    //                 Ok(n) if n == 0 => {
    //                     println!("EOF reached");
    //                     break 'buff_loop;
    //                 }
    //                 Ok(_) => {
    //                     match file.write_all(&buf) {
    //                         Ok(_) => {}
    //                         Err(e) => eprintln!("Problem writing to file: {}", e),
    //                     };
    //                 }
    //                 Err(e) => {
    //                     eprintln!("failed to read from socket; err = {:?}", e);
    //                     break 'buff_loop;
    //                 }
    //             };
    //         }
    //         file.flush().unwrap();

    //         println!("Did we ever get here ?");

    //         let output = server::executor::execute_bin("test_executable");

    //         // Write IO output back to the socket
    //         if let Err(e) = socket.write_all(output.as_bytes()).await {
    //             eprintln!("failed to write to socket; err = {:?}", e);
    //         }
    //     });
    // }

    Ok(())
}

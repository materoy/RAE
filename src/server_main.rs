use std::fmt::Debug;
use std::os::unix::prelude::AsRawFd;

use application_proto::stream_service_server::{StreamService, StreamServiceServer};
use application_proto::{ApplicationRequest, ApplicationResponse, Input};
use nix::libc::close;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
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
        let mut file = server::file_io::create_bin_file(&file_path).await;

        file.write_all(&req.executable)
            .await
            .expect("Failed to write to file");

        unsafe {
            let exit_status = close(file.as_raw_fd());
            println!("Close exit Status: {}", exit_status);
        }
        drop(file);

        let message = match server::executor::execute_bin(
            &file_path,
            req.argv.iter().map(|s| &**s).collect(),
        ) {
            Some(mut child) => {
                let mut stdout = child.stdout.take().expect("no stdout");
                let mut output = String::new();
                stdout.read_to_string(&mut output).await.unwrap();
                println!("Output: {}", output);
                output
            }
            None => {
                eprintln!("No output");
                String::from("No output")
            }
        };

        // Deletes the generated bin file
        server::file_io::delete_file(&file_path).await;

        Ok(Response::new(ApplicationResponse { result: message }))
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

    Ok(())
}

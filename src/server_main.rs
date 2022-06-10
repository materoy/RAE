
use std::fmt::Debug;
use std::io::Write;
use std::process::exit;

use application_proto::stream_service_server::{StreamService, StreamServiceServer};
use application_proto::{ApplicationRequest, ApplicationResponse, Input};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::task::futures;
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
            .expect("Failed to write to file");
        drop(file);

        // unsafe {
        //     let exit_status = close(file.as_raw_fd());
        //     println!("Close exit Status: {}", exit_status);
        // }

        let mut child =
            server::executor::execute_bin(&file_path, req.argv.iter().map(|s| &**s).collect())
                .unwrap();
        println!("Execution Started... {}", file_path);

        let output = match child.stdout.take() {
            Some(mut child_stdout) => {
                let mut output = String::new();
                println!("OUTPUT: {:?}", child_stdout);
                // match stdout.read_to_string(&mut output) {
                //     Ok(_) => todo!(),
                //     Err(_) => todo!(),
                // }
                // let mut buf: Vec<u8> = Vec::new();

                let lines_read = child_stdout.read_to_string(&mut output).await.unwrap();
                println!("{}", lines_read);

                // output
                "Hello".to_string()
            }
            None => String::from("NO OUTPUT"),
        };

        /* Just if the data is not empty push it to stdin */
        if req.data != String::from("") {
            let stdin = child.stdin.as_mut().unwrap();
            stdin.write_all(req.data.as_bytes()).await.unwrap();
        }

        println!("Reached here");

        // Deletes the generated bin file
        server::file_io::delete_file_async(&file_path).await;

        Ok(Response::new(ApplicationResponse { result: output }))
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

    ctrlc::set_handler(move || {
        println!("SIGTERM received, cleaning up...");
        server::file_io::delete_all_in_dir("bin");
        println!("Clean up complete. Shutting down with exit code 1");
        exit(1);
    })?;

    let addr = "127.0.0.1:5050".parse()?;

    let application_service = ApplicationService::default();

    Server::builder()
        .add_service(StreamServiceServer::new(application_service))
        .serve(addr)
        .await?;

    Ok(())
}

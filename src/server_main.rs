use std::fmt::Debug;
use std::io::Write;
use std::process::exit;
use std::sync::mpsc::{self, Receiver, Sender};

use application_proto::stream_service_server::{StreamService, StreamServiceServer};
use application_proto::{ApplicationRequest, ApplicationResponse, Input};
use bytes::BufMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tonic::{transport::Server, Request, Response, Status};
mod application;
mod server;

pub mod application_proto {
    tonic::include_proto!("application");
}

pub mod consts;

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

        let mut child =
            server::executor::execute_bin(&file_path, req.argv.iter().map(|s| &**s).collect())
                .unwrap();
        println!("Execution Started... {}", file_path);

        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

        tokio::spawn(async move {
            let output = match child.stdout.take() {
                Some(mut child_stdout) => {
                    let mut buf = [0; 1024];

                    let lines_read = child_stdout.read(&mut buf).await.unwrap();
                    println!("Lines read from CHILDSTDOUT: {}", lines_read);
                    let string_output =
                        std::str::from_utf8(&buf[0..lines_read]).expect("Failed to convert output to string");
                    println!("OUTPOUT: {}", string_output);
                    string_output.to_owned()
                }
                None => String::from("NO OUTPUT"),
            };
            println!("Output is: {}", output);
            tx.send(output).expect("Could not send to receiver");
        });

        tokio::spawn(async move {
            /* Just if the data is not empty push it to stdin */
            if !req.data.is_empty() {
                let stdin = child.stdin.as_mut().unwrap();
                println!("CHILDSTDIN");
                stdin.write_all(req.data.as_bytes()).await.unwrap();
            }
        });

        println!("Reached here");

        // Deletes the generated bin file
        server::file_io::delete_file_async(&file_path).await;

        Ok(Response::new(ApplicationResponse {
            result: rx.recv().expect("Failed to receive on tx"),
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
    ctrlc::set_handler(move || {
        server::file_io::delete_all_in_dir("bin");
        println!("SIGTERM received, cleaning up...");
        server::file_io::delete_all_in_dir("bin");
        println!("Clean up complete. Shutting down with exit code 1");
        exit(1);
    })?;

    let addr = consts::ADDRESS.parse()?;

    let application_service = ApplicationService::default();

    Server::builder()
        .add_service(StreamServiceServer::new(application_service))
        .serve(addr)
        .await?;

    Ok(())
}

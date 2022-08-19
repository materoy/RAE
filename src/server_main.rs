use std::fmt::Debug;
use std::io::Write;
use std::pin::Pin;
use std::process::exit;
use tokio::sync::mpsc;

use application_proto::stream_service_server::{StreamService, StreamServiceServer};
use application_proto::{ApplicationRequest, ApplicationResponse};
use futures::Stream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
mod application;
mod file_io;
mod server;

pub mod application_proto {
    tonic::include_proto!("application");
}

pub mod consts;

type ApplicationResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<ApplicationResponse, Status>> + Send>>;

#[derive(Debug, Default)]
pub struct RaeServer {}

#[tonic::async_trait]
impl StreamService for RaeServer {
    type StartApplicationStream = ResponseStream;

    async fn start_application(
        &self,
        request: Request<ApplicationRequest>,
    ) -> ApplicationResult<Self::StartApplicationStream> {
        println!("Client connected from: {:?}", request.remote_addr());

        let req = request.into_inner();
        println!("{}", req.name);

        let file_path = format!("bin/{}", req.name);
        let mut file = file_io::create_bin_file(&file_path).await;

        file.write_all(&req.executable)
            .expect("Failed to write to file");
        drop(file);

        let mut child =
            server::executor::execute_bin(&file_path, req.argv.iter().map(|s| &**s).collect())
                .unwrap();
        println!("Execution Started... {}", file_path);

        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            if let Some(ref mut child_stdout) = child.stdout.take() {
                let mut buf = [0; 1024];

                let lines_read = &child_stdout.read(&mut buf).await.unwrap();
                println!("Lines read from CHILDSTDOUT: {}", lines_read);
                let string_output = std::str::from_utf8(&buf[0..*lines_read])
                    .expect("Failed to convert output to string");
                println!("Output is: {}", string_output);
                let resp = ApplicationResponse {
                    result: string_output.to_owned(),
                };

                match tx.send(Result::<_, Status>::Ok(resp)).await {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error queuing to stream: {}", e)
                    }
                }
            } else {
                println!("NO OUTPUT");
                // // Deletes the generated bin file
                // server::file_io::delete_file_async(&file_path).await;
            }
        });

        tokio::spawn(async move {
            /* Just if the data is not empty push it to stdin */
            if !req.data.is_empty() {
                let stdin = child.stdin.as_mut().unwrap();
                println!("CHILDSTDIN");
                stdin.write_all(req.data.as_bytes()).await.unwrap();
            }
        });

        let out_stream = ReceiverStream::new(rx);

        Ok(Response::new(
            Box::pin(out_stream) as Self::StartApplicationStream
        ))
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(exit_interupt_handler)?;

    let addr = consts::ADDRESS.parse()?;

    let application_service = RaeServer::default();

    Server::builder()
        .add_service(StreamServiceServer::new(application_service))
        .serve(addr)
        .await?;

    Ok(())
}

fn exit_interupt_handler() {
    file_io::delete_all_in_dir("bin");
    println!("SIGTERM received, cleaning up...");
    file_io::delete_all_in_dir("bin");
    println!("Clean up complete. Shutting down with exit code 1");
    exit(1);
}

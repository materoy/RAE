use std::env;

use application_proto::stream_service_client::StreamServiceClient;
use application_proto::ApplicationRequest;

mod client;

pub mod application_proto {
    tonic::include_proto!("application");
}

pub mod consts;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().skip(1).collect::<Vec<String>>();
    // let default_program_path = "target/debug/sample-hello-world";
    let default_program_path = "target/debug/sample-input-echo";
    let program_path = match args.first() {
        Some(title) => match title.as_str() {
            "input-echo" => "target/debug/sample-input-echo",
            _ => default_program_path,
        },
        None => default_program_path,
    };

    let scheme = "http://";
    let addr = scheme.to_owned() + consts::ADDRESS;

    let mut application_client = StreamServiceClient::connect(addr).await?;

    // Send binary file to server
    let bin = &client::file_io::read_bin_file(program_path);

    // let mut input = String::new();
    // stdin().read_line(&mut input).unwrap();

    let request = tonic::Request::new(ApplicationRequest {
        name: String::from(program_path.split('/').last().unwrap()),
        executable: bin.to_vec(),
        execute_command: String::from("./"),
        data: String::from("Just input"),
        path: String::from(""),
        argv: Vec::new(),
        envv: Vec::new(),
    });

    let response = application_client.start_application(request).await?;

    let mut stream = response.into_inner();

    while let Some(item) = stream.message().await? {
        println!("GOT SOME RESPONSE !");
        println!("{}", item.result);
    }

    Ok(())
}

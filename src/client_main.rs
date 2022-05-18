use application_proto::stream_service_client::StreamServiceClient;
use application_proto::ApplicationRequest;

mod client;

pub mod application_proto {
    tonic::include_proto!("application");
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr = "http://127.0.0.1:5050";
    let mut application_client = StreamServiceClient::connect(server_addr).await?;

    let request = tonic::Request::new(ApplicationRequest {
        name: String::from("What a name"),
        executable: vec![],
        execute_command: String::from(""), 
        data: String::from(""),
        path: String::from(""),
        argv: Vec::new(),
        envv: Vec::new(), 
    });

    let response = application_client.start_application(request).await?;

    println!("Response: {:?}", response);

    Ok(())

    // if let Ok(mut socket) = TcpStream::connect(server_addr).await {
    //     println!("Connected to server");

    //     if let Err(e) = socket.write(b"Hello world").await {
    //         eprintln!("failed to write to socket; err = {:?}", e);
    //     }

    //     let mut buf = [0; 1024];

    //     // Send binary file to server
    //     let path = "exec/target/release/exec";
    //     match socket.write_all(&client::file_io::read_bin_file(path)).await {
    //         Ok(_) => {
    //             // println!("{} bytes sent to server", n);
    //         }
    //         Err(e) => {
    //             eprintln!("Error sending to server: {}", e);
    //         }
    //     }

    //     socket.flush().await.unwrap();

    //     println!("File sent to server.. wait for reply");

    //     let n = match socket.read(&mut buf).await {
    //         Ok(n) if n == 0 => return,
    //         Ok(n) => n,
    //         Err(e) => {
    //             eprintln!("failed to read from socket; err = {:?}", e);
    //             return;
    //         }
    //     };
    //     println!("Read {} bytes from server", n);

    //     if n > 0 {
    //         println!("{}", String::from_utf8_lossy(&buf))
    //     }
    // } else {
    //     println!("Couldn't connect to server");
    // }
}

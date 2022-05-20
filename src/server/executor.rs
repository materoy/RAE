use std::process::{Command, Output};

/*
    This command executes a binary given its path
    then returns and prints out its's output
*/
pub fn execute_bin(path: &str, args: Vec<&str>) -> Option<Output> {
    let output = match Command::new(path).args(args).spawn() {
        Ok(child) => match child.wait_with_output() {
            Ok(output) => Some(output),
            Err(err) => {
                eprintln!("ERROR RUNNING BINRARY: {}", err);
                None
            }
        },
        Err(err) => {
            eprintln!("ERROR RUNNING BINRARY: {}", err);
            None
        }
    };

    output
}

// pub fn execute_command(program: &str, args: Vec<&str>) -> String {
//     let mut command = Command::new(program);

//     println!(
//         "Executing {} {:?}...",
//         command.get_program().to_str().unwrap(),
//         args
//     );
//     match command.args(&args).output() {
//         Ok(output) => {
//             let output_string = String::from_utf8_lossy(&output.stdout);
//             println!("Output {:?}", output_string);
//             output_string.to_string()
//         }
//         Err(e) => {
//             eprintln!("ERROR RUNNING COMMAND: {}", e);
//             String::from("Some problem here your application could not be executed")
//         }
//     }
// }

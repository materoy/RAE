use std::process::Command;

/*
    This command executes a binary given its path
    then returns and prints out its's output
*/
pub fn execute_bin(path: &str) -> String {
    let mut command = Command::new(format!("./{}", path));

    println!("Executing {}...", command.get_program().to_str().unwrap());
    match command.output() {
        Ok(output) => {
            let output_string = String::from_utf8_lossy(&output.stdout);
            println!("Output {:?}", output_string);
            output_string.to_string()
        }
        Err(e) => {
            eprintln!("ERROR RUNNING BINRARY: {}", e);
            String::from("Some problem here your application could not be executed")
        }
    }
}

pub fn execute_command(program: &str, args: Vec<&str>) -> String {
    let mut command = Command::new(program);

    println!("Executing {} {:?}...", command.get_program().to_str().unwrap(), args);
    match command.args(&args).output() {
        Ok(output) => {
            let output_string = String::from_utf8_lossy(&output.stdout);
            println!("Output {:?}", output_string);
            output_string.to_string()
        }
        Err(e) => {
            eprintln!("ERROR RUNNING COMMAND: {}", e);
            String::from("Some problem here your application could not be executed")
        }
    }
}
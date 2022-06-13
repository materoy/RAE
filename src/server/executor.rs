use std::process::Stdio;

use tokio::process::{Child, Command};

/*
    This command executes a binary given its path
    then returns and prints out its's output
*/
pub fn execute_bin(path: &str, args: Vec<&str>) -> Option<Child> {
    match Command::new(path)
        .args(args)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => Some(child),
        Err(err) => {
            eprintln!("ERROR RUNNING BINRARY: {}", err);
            None
        }
    }
}

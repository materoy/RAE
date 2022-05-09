use std::process::Command;

fn main() {
    let exec_dir = "./exec/target/release/exec";
    let mut command = Command::new(exec_dir);

    println!("Executing {}...", exec_dir);
    command.status().expect("Failed to execute command");
}

use std::process::Command;

fn main() {
    let mut command = Command::new("lasfjl");

    command.status().expect("Failed to execute command");

}

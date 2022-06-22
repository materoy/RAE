/// This sample just takes input from stdio
/// and spits it back to stdout
/// infinitely

fn main() {
    let mut input_string = String::new();
    loop {
        match std::io::stdin().read_line(&mut input_string) {
            Ok(_) => println!("{}", input_string),
            Err(e) => eprintln!("Failed to read from stdin: {}", e),
        }

        // clear buffer before next pass
        input_string = String::from("");
    }
}

use std::io::stdin;

fn main() {
    println!("Hi this is echo server");
    let mut input_string = String::new();
    match stdin().read_line(&mut input_string) {
        Ok(_) => println!("{}", input_string),
        Err(e) => eprintln!("Failed to read from stdin: {}", e),
    }
}

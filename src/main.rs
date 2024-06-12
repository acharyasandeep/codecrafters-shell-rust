#[allow(unused_imports)]
use std::io::{self, Write};

fn handle_commands(input: String) {
    let input_split: Vec<_> = input.split(" ").collect();
    let command = input_split[0].trim();

    match command {
        "" => {}
        _ => {
            println!("{}: command not found", command);
        }
    }
}

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        let bytes_read = stdin.read_line(&mut input).unwrap();

        if bytes_read == 0 {
            break;
        }

        handle_commands(input)
    }
}

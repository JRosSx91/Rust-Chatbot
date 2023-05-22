use std::io::{self, Write};

fn main() {
    println!("Hello, I'm your ChatBot!");

    loop {
        print!("> ");
        let mut stdout: io::Stdout = io::stdout();
        stdout.flush().unwrap();

        let mut input: String = String::new();
        let stdin: io::Stdin = io::stdin();
        stdin.read_line(&mut input).unwrap();

        let input: &str = input.trim();
    }
}

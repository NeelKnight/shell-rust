#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    //println!("Welcome to the rush (RUst SHell!");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // !TODO try to reduce streaming vulnerabilites
        let sanitised_input = input.trim();

        match sanitised_input {
            "exit 0" => break,
            _ => println!("{}: command not found", sanitised_input),
        }
    }
}

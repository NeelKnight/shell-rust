use std::io::{self, Write};

fn main() {
    println!("Welcome to the rush (RUst SHell)!");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // !TODO try to reduce streaming vulnerabilites
        let sanitised_input = sanitise_input(&input);

        let mut parts = sanitised_input.split_whitespace();

        match parts.next() {
            Some("exit") => match parts.next() {
                Some(code) => {
                    std::process::exit(code.parse::<i32>().unwrap_or(1));
                }
                None => std::process::exit(1),
            },
            Some("echo") => println!("{}", parts.collect::<Vec<&str>>().join(" ")),
            None => continue,
            _ => println!("{}: command not found", sanitised_input),
        }
    }
}

fn sanitise_input(input: &str) -> &str {
    input.trim()
}

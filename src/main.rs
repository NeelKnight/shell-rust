use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
    vec,
};

fn main() {
    //println!("Welcome to the rush (RUst SHell)!");

    let path = env::var("PATH").unwrap_or_default();
    let path_directories = fetch_path_dir(&path);

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // !TODO try to reduce streaming vulnerabilites
        let sanitised_input = sanitise_input(&input);

        let mut parts = sanitised_input.split_whitespace();
        let built_in_commands = vec!["type", "echo", "exit"];

        match parts.next() {
            Some("exit") => match parts.next() {
                Some(code) => {
                    std::process::exit(code.parse::<i32>().unwrap_or(1));
                }
                None => std::process::exit(1),
            },
            Some("echo") => println!("{}", parts.collect::<Vec<&str>>().join(" ")),
            Some("type") => match parts.next() {
                Some(command) => {
                    if built_in_commands.contains(&command) {
                        println!("{command} is a shell builtin")
                    } else if let Some(filepath) =
                        search_commands_in_path(command, &path_directories)
                    {
                        println!("{command} is {}", filepath.display());
                    } else {
                        println!("{command}: not found")
                    }
                }
                None => continue,
            },
            None => continue,
            _ => println!("{}: command not found", sanitised_input),
        }
    }
}

fn sanitise_input(input: &str) -> &str {
    input.trim()
}

fn fetch_path_dir(path: &str) -> Vec<&str> {
    return path.split(":").collect();
}

fn search_commands_in_path(command: &str, directories: &[&str]) -> Option<PathBuf> {
    for dir in directories {
        if let Ok(files) = fs::read_dir(dir) {
            for file in files {
                let Ok(file) = file else { continue };

                if let Some(file_name) = file.file_name().to_str() {
                    if file_name == command {
                        return Some(file.path());
                    }
                }
            }
        }
    }
    None
}

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

    let built_in_commands = vec!["type", "echo", "exit"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // !TODO try to reduce streaming vulnerabilites
        let sanitised_input = match sanitise_input(&input) {
            Some(value) => value,
            None => {
                eprintln!("Attack Vector discovered! Input discarded as invalid!");
                return;
            }
        };

        let mut parts = sanitised_input.split_whitespace();
        let command = parts.next();
        let args: Vec<&str> = parts.collect();

        match command {
            Some("exit") => match args.get(0) {
                Some(code) => {
                    std::process::exit(code.parse::<i32>().unwrap_or(1));
                }
                None => std::process::exit(1),
            },
            Some("echo") => println!("{}", args.join(" ")),
            Some("type") => match args.get(0) {
                Some(command_provided) => {
                    if built_in_commands.contains(&command_provided) {
                        println!("{command_provided} is a shell builtin")
                    } else if let Some(filepath) =
                        search_command_in_path(command_provided, &path_directories)
                    {
                        println!("{command_provided} is {}", filepath.display());
                    } else {
                        println!("{command_provided}: not found")
                    }
                }
                None => continue,
            },
            Some(command) => {
                if let Some(_) = search_command_in_path(command, &path_directories) {
                    if let Err(e) = execute_command(command, &args) {
                        eprintln!("Failed to execute command: {}", e);
                    }
                } else {
                    println!("{}: command not found", command);
                }
            }
            None => continue,
        }
    }
}

// !TODO try to reduce streaming vulnerabilites
fn sanitise_input(input: &str) -> Option<String> {
    let trimmed = input.trim();

    // Dis-allow Control Characters to prevent weird shell effects!
    if trimmed
        .chars()
        .any(|c| c.is_control() && !c.is_whitespace())
    {
        return None;
    }

    // Prevent Resoure exhaustion attacks
    if trimmed.len() > 1024 {
        return None;
    }

    Some(trimmed.split_whitespace().collect::<Vec<_>>().join(" "))
}

fn fetch_path_dir(path: &str) -> Vec<&str> {
    return path.split(":").collect();
}

fn search_command_in_path(command: &str, directories: &[&str]) -> Option<PathBuf> {
    for dir in directories {
        if let Ok(dir_entries) = fs::read_dir(dir) {
            for dir_entry in dir_entries.flatten() {
                let filepath = dir_entry.path();

                if filepath.is_file() {
                    if let Some(filename) = filepath.file_name() {
                        if filename == command {
                            return Some(filepath);
                        }
                    }
                }
            }
        }
    }
    None
}

fn execute_command(command: &str, args: &[&str]) -> io::Result<()> {
    let output = std::process::Command::new(command)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()?;

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}

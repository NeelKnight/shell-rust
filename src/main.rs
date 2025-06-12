use dirs;
use std::{
    env, fs,
    io::{self, Write},
    path::{self, PathBuf},
};

// Start of add of command modularisation

fn main() {
    //println!("Welcome to the rush (RUst SHell)!");

    let path = env::var("PATH").unwrap_or_default();
    let path_directories = fetch_path_dir(&path);

    const BUILT_IN_COMMANDS: [&str; 5] = ["type", "echo", "exit", "pwd", "cd"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // !TODO try to reduce streaming vulnerabilites
        let sanitised_input = match sanitise_input(&input) {
            Some(value) => value,
            None => {
                println!("Attack Vector discovered! Input discarded as invalid!");
                continue;
            }
        };

        let (command, args) = match sanitised_input.split_first() {
            Some((cmd, rest)) => (
                Some(cmd.as_str()),
                rest.iter().map(|rest_part| rest_part.as_str()).collect(),
            ),
            None => (None, Vec::new()),
        };

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
                    if BUILT_IN_COMMANDS.contains(&command_provided) {
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
            Some("pwd") => match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(error) => println!("Failed to fetch directory : {error}!"),
            },
            // !TODO Improve cd by adding '-' and ' ' support
            Some("cd") => {
                let combined_dir_path = args.join(" ");

                let to_dir = if combined_dir_path.starts_with("~") {
                    if let Some(home_dir) = dirs::home_dir() {
                        let mut path = home_dir;

                        let suffix = combined_dir_path
                            .trim_start_matches("~")
                            .trim_start_matches("/");

                        if !suffix.is_empty() {
                            path.push(suffix);
                        }
                        Some(path)
                    } else {
                        println!("Home directory not set!");
                        None
                    }
                } else {
                    Some(path::PathBuf::from(&combined_dir_path))
                };

                if let Some(dir) = to_dir {
                    if env::set_current_dir(&dir).is_err() {
                        println!("cd: {}: No such file or directory", dir.display())
                    }
                }
            }
            Some(command) => {
                if let Some(_) = search_command_in_path(command, &path_directories) {
                    if let Err(e) = execute_command(command, &args) {
                        println!("Failed to execute command: {}", e);
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
fn sanitise_input(input: &str) -> Option<Vec<String>> {
    let trimmed = input.trim();

    // Dis-allow Control Characters to prevent weird shell effects!
    if trimmed
        .chars()
        .any(|c| c.is_control() && !c.is_whitespace())
    {
        return None;
    }

    // Prevent Resource exhaustion attacks
    if trimmed.len() > 1024 {
        return None;
    }

    match shell_words::split(trimmed) {
        Ok(parts) => Some(parts),
        Err(error) => {
            eprintln!("Input parsing failed due to mismatched-quotes: {error}");
            None
        }
    }
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

// Fine tune this and ensure proper error handling
fn execute_command(command: &str, args: &[&str]) -> io::Result<()> {
    let mut process = std::process::Command::new(command).args(args).spawn()?;

    let _status = process.wait()?;

    Ok(())
}

// !TODO
// try to reduce streaming vulnerabilites
// Improve cd by adding '-' and ' ' support
// Fine tune execute_command this and ensure proper error handling
// Error reporting in red?
// Backspace and navigation support
// Pretty-print support?

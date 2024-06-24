#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use std::{env, fs};

const SHELL_BUILTINS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

fn check_executable(command: String) -> (bool, String) {
    let search_directories = env::var("PATH").unwrap();
    let search_directories: Vec<_> = search_directories.split(":").collect();
    let (mut is_executable, mut path_to_executable) = (false, String::from(""));

    for mut directory in search_directories {
        directory = directory.trim();
        let all_files = fs::read_dir(directory);

        match all_files {
            Ok(_all_files) => {
                let files: Vec<_> = _all_files.collect();
                for file in files {
                    let file = file.unwrap();
                    let file_name = file.file_name().into_string().unwrap();
                    if command == file_name {
                        is_executable = true;
                        let file_path_clone = file.path().clone();
                        let path_to_executable_str = file_path_clone.to_str().unwrap();
                        path_to_executable = path_to_executable_str.to_string();
                        break;
                    }
                }
            }
            Err(e) => {
                println!("can't read {}, error: {:?}", directory, e)
            }
        }
    }

    return (is_executable, path_to_executable);
}

fn handle_commands(input: String) {
    let input_split: Vec<_> = input.split(" ").collect();
    let command = input_split[0].trim();

    match command {
        "exit" => {
            if input_split.len() < 2 {
                panic!("exit code not supplied");
            }
            let exit_code = input_split[1];
            let exit_code: i32 = exit_code.parse().unwrap_or_default();
            std::process::exit(exit_code);
        }
        "echo" => {
            let mut string_to_print = String::from("");
            if input_split.len() >= 2 {
                string_to_print = input_split[1..].join(" ").trim().to_string();
            }
            println!("{}", string_to_print);
        }
        "type" => {
            if input_split.len() < 2 {
                print!("wrong no of arguments; no command to return the type of");
                return;
            }

            let arg = input_split[1].trim();
            if SHELL_BUILTINS.contains(&arg) {
                println!("{} is a shell builtin", arg);
            } else {
                let (is_executable, path_to_executable) = check_executable(arg.to_string());
                if is_executable {
                    println!("{} is {}", arg, path_to_executable);
                } else {
                    println!("{}: not found", arg);
                }
            }
        }
        "pwd" => {
            let current_dir = env::current_dir().unwrap();
            let current_dir_str = current_dir.to_string_lossy();
            println!("{}", current_dir_str);
        }
        "cd" => {
            if input_split.len() < 2 {
                println!("wrong number of arguments; no path supplied");
                return;
            }
            let mut path_to_change = input_split[1].trim().to_string();
            if path_to_change == "~" {
                let home_dir = env::var("HOME");
                let home_dir = home_dir.unwrap_or_else(|_| panic!("HOME directory not set"));
                path_to_change = home_dir
            }

            let result = env::set_current_dir(&path_to_change);
            if result.is_err() {
                println!("cd: {}: No such file or directory", path_to_change);
            }
        }
        "" => {}
        _ => {
            let (is_executable, path_to_executable) = check_executable(command.to_string());
            if is_executable {
                let mut args = vec![];

                for i in 1..input_split.len() {
                    args.push(input_split[i].trim());
                }

                let output = Command::new(path_to_executable)
                    .args(args)
                    .output()
                    .expect("can't execute command");
                let error = String::from_utf8_lossy(&output.stderr);
                let output = String::from_utf8_lossy(&output.stdout);

                if error.len() > 0 {
                    print!("{}", error);
                }

                print!("{}", output);
            } else {
                println!("{}: command not found", command);
            }
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

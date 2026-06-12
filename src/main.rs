use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    process::{Command, Stdio},
};
mod builtin;
mod lexer;
mod utils;

use builtin::Builtin;

use crate::{lexer::tokenize, utils::find_in_path};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let cmd_token = tokenize(input);
        if cmd_token.is_empty() {
            continue;
        }

        let (cmd, raw_args) = (cmd_token[0].as_str(), cmd_token[1..].to_vec());

        let mut actual_args = Vec::new();
        let mut output_file = None;
        let mut error_output_file = None;
        let mut to_append_output = false;
        let mut i = 0;

        while i < raw_args.len() {
            if raw_args[i] == ">" || raw_args[i] == "1>" {
                if i + 1 < raw_args.len() {
                    output_file = Some(raw_args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Syntax error: expected file after '{}'", raw_args[i]);
                    break;
                }
            } else if raw_args[i] == "2>" {
                if i + 1 < raw_args.len() {
                    error_output_file = Some(raw_args[i + 1].clone());
                    i += 2;
                }
            } else if raw_args[i] == ">>" || raw_args[i] == "1>>" {
                output_file = Some(raw_args[i + 1].clone());
                to_append_output = true;
                i += 2;
            } else if raw_args[i] == "2>>" {
                error_output_file = Some(raw_args[i + 1].clone());
                to_append_output = true;
                i += 2;
            } else {
                actual_args.push(raw_args[i].clone());
                i += 1;
            }
        }

        if let Some(builtin) = Builtin::parse(cmd) {
            if let Some(ref file_path) = output_file {
                if to_append_output {
                    let _ = OpenOptions::new().create(true).append(true).open(file_path);
                } else {
                    let _ = File::create(file_path);
                }
            }
            if let Some(ref file_path) = error_output_file {
                if to_append_output {
                    let _ = OpenOptions::new().create(true).append(true).open(file_path);
                } else {
                    let _ = File::create(file_path);
                }
            }
            match builtin.execute(actual_args) {
                Ok(output) => {
                    if let Some(file_path) = output_file {
                        let file_result = if to_append_output {
                            OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(&file_path)
                        } else {
                            File::create(&file_path)
                        };

                        match file_result {
                            Ok(mut file) => match writeln!(file, "{}", output) {
                                Ok(_) => {}
                                Err(e) => eprintln!("Error writing to file: {}", e),
                            },
                            Err(e) => eprintln!("Error opening file {}: {}", file_path, e),
                        }
                    } else {
                        if !output.is_empty() {
                            println!("{}", output);
                        }
                    }
                }
                Err(e) => {
                    if let Some(file_path) = error_output_file {
                        let file_result = if to_append_output {
                            OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(&file_path)
                        } else {
                            File::create(&file_path)
                        };

                        match file_result {
                            Ok(mut file) => match writeln!(file, "{}", e) {
                                Ok(_) => {}
                                Err(e) => eprintln!("Error writing to file: {}", e),
                            },
                            Err(e) => eprintln!("Error opening file {}: {}", file_path, e),
                        }
                    } else {
                        eprintln!("{}", e)
                    }
                }
            }
            continue;
        } else {
            if let Some(_path) = find_in_path(cmd) {
                let mut command = Command::new(cmd);
                command.args(actual_args);

                if let Some(file_path) = output_file {
                    let file_result = if to_append_output {
                        OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&file_path)
                    } else {
                        File::create(&file_path)
                    };

                    match file_result {
                        Ok(file) => {
                            command.stdout(Stdio::from(file));
                        }
                        Err(e) => {
                            eprintln!("Error opening file {}: {}", file_path, e);
                            continue;
                        }
                    }
                }

                if let Some(file_path) = error_output_file {
                    let file_result = if to_append_output {
                        OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&file_path)
                    } else {
                        File::create(&file_path)
                    };
                    match file_result {
                        Ok(file) => {
                            command.stderr(Stdio::from(file));
                        }
                        Err(e) => {
                            eprintln!("Error opening file {}: {}", file_path, e);
                            continue;
                        }
                    }
                }

                match command.spawn() {
                    Ok(mut child_process) => {
                        let _ = child_process.wait();
                    }
                    Err(e) => {
                        eprintln!("Error executing command: {}", e);
                    }
                }
                continue;
            }
        }

        println!("{}: command not found", cmd);
    }
}

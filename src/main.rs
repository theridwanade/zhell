use std::{
    fs::File,
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
            } else {
                actual_args.push(raw_args[i].clone());
                i += 1;
            }
        }

        if let Some(builtin) = Builtin::parse(cmd) {
            match builtin.execute(actual_args) {
                Ok(output) => {
                    if output.is_empty() {
                        continue;
                    }
                    if let Some(file_path) = output_file {
                        match File::create(&file_path) {
                            Ok(mut file) => match writeln!(file, "{}", output) {
                                Ok(_) => {}
                                Err(e) => eprintln!("Error writing to file: {}", e),
                            },
                            Err(e) => eprintln!("Error opening file {}: {}", file_path, e),
                        }
                    } else {
                        println!("{}", output);
                    }
                }
                Err(e) => eprintln!("Error executing builtin: {}", e),
            }
            continue;
        } else {
            if let Some(_path) = find_in_path(cmd) {
                let mut command = Command::new(cmd);
                command.args(actual_args);

                if let Some(file_path) = output_file {
                    match File::create(&file_path) {
                        Ok(file) => {
                            command.stdout(Stdio::from(file));
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

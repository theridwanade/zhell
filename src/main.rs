use std::{
    io::{self, Write},
    process::Command,
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

        let (cmd, args) = match input.split_once(" ") {
            Some((c, a)) => (c, tokenize(a)),
            None => (input, Vec::new()),
        };

        if let Some(builtin) = Builtin::parse(cmd) {
            builtin.execute(args);
            continue;
        } else {
            if let Some(_path) = find_in_path(cmd) {
                let child = Command::new(cmd).args(args).spawn();

                match child {
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

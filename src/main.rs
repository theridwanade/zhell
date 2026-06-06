use std::{
    io::{self, Write},
    process::Command,
};
mod builtin;
mod utils;

use builtin::Builtin;

use crate::utils::find_in_path;

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
            Some((c, a)) => (c, a.trim()),
            None => (input, ""),
        };

        if let Some(builtin) = Builtin::parse(cmd) {
            builtin.execute(args);
            continue;
        } else {
            if let Some(_path) = find_in_path(cmd) {
                let cmd_args = args.split_whitespace();

                let child = Command::new(cmd).args(cmd_args).spawn();

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

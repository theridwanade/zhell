use std::env;

use crate::utils::find_in_path;

pub enum Builtin {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
}

impl Builtin {
    pub fn parse(cmd: &str) -> Option<Self> {
        match cmd {
            "exit" => Some(Builtin::Exit),
            "echo" => Some(Builtin::Echo),
            "type" => Some(Builtin::Type),
            "pwd" => Some(Builtin::Pwd),
            "cd" => Some(Builtin::Cd),
            _ => None,
        }
    }

    pub fn execute(&self, args: &str) {
        match self {
            Builtin::Exit => std::process::exit(0),
            Builtin::Echo => println!("{}", args),
            Builtin::Type => {
                if Builtin::parse(args).is_some() {
                    println!("{} is shell builtin", args);
                } else if let Some(path) = find_in_path(args) {
                    println!("{} is {}", args, path.display());
                } else {
                    println!("{}: not found", args);
                }
            }
            Builtin::Pwd => {
                if let Ok(current_dir) = env::current_dir() {
                    println!("{}", current_dir.display());
                } else {
                    eprintln!("Error getting current directory");
                }
            }
            Builtin::Cd => {
                let target_dir = if args.is_empty() {
                    match env::var("HOME") {
                        Ok(home) => home,
                        Err(_) => ".".to_string(),
                    }
                } else {
                    args.to_string()
                };

                match env::set_current_dir(&target_dir) {
                    Ok(_) => {}
                    Err(_e) => eprintln!("cd: {}: No such file or directory", target_dir),
                }
            }
        }
    }
}

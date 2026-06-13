use std::{
    env,
    io::{Error, ErrorKind},
};

use crate::utils::{find_in_path, get_current_working_directory};

pub enum Builtin {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
}

impl Builtin {
    pub const COMMANDS: [&'static str; 5] = ["exit", "echo", "type", "pwd", "cd"];
    
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

    pub fn execute(&self, args: Vec<String>) -> Result<String, Error> {
        let args = args.join(" ");
        match self {
            Builtin::Exit => std::process::exit(0),
            Builtin::Echo => Ok(format!("{}", args)),
            Builtin::Type => {
                if Builtin::parse(args.as_str()).is_some() {
                    Ok(format!("{} is a shell builtin", args))
                } else if let Some(path) = find_in_path(args.as_str()) {
                    Ok(format!("{} is {}", args, path.display()))
                } else {
                    Ok(format!("{}: not found", args))
                }
            }
            Builtin::Pwd => get_current_working_directory(),
            Builtin::Cd => {
                let target_dir = if args.is_empty() || args == "~" {
                    match env::var("HOME") {
                        Ok(home) => home,
                        Err(_) => ".".to_string(),
                    }
                } else if args.starts_with("~/") {
                    match env::var("HOME") {
                        Ok(home) => format!("{}{}", home, &args[1..]),
                        Err(_) => args.to_string(),
                    }
                } else if args.starts_with("-") {
                    match env::var("OLDPWD") {
                        Ok(oldpwd) => oldpwd,
                        Err(_) => {
                            return Err(Error::new(ErrorKind::Other, "cd: OLDPWD not set"));
                        }
                    }
                } else {
                    args.to_string()
                };

                let previous_dir = match env::current_dir() {
                    Ok(dir) => dir,
                    Err(_) => {
                        return Err(Error::new(
                            ErrorKind::Other,
                            "Error getting current directory",
                        ));
                    }
                };

                match env::set_current_dir(&target_dir) {
                    Ok(_) => {
                        unsafe {
                            env::set_var("OLDPWD", previous_dir);
                        }
                        Ok(String::new())
                    }
                    Err(_) => Err(Error::new(
                        ErrorKind::NotFound,
                        format!("cd: {}: No such file or directory", target_dir),
                    )),
                }
            }
        }
    }
}

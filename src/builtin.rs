use std::{
    env,
    io::{Error, ErrorKind},
};

use rustyline::{
    Editor,
    history::{DefaultHistory},
};

use crate::utils::{ZhellHelper, find_in_path};

pub enum Builtin {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
    History,
}

impl Builtin {
    pub const COMMANDS: [&'static str; 6] = ["exit", "echo", "type", "pwd", "cd", "history"];

    pub fn parse(cmd: &str) -> Option<Self> {
        match cmd {
            "exit" => Some(Builtin::Exit),
            "echo" => Some(Builtin::Echo),
            "type" => Some(Builtin::Type),
            "pwd" => Some(Builtin::Pwd),
            "cd" => Some(Builtin::Cd),
            "history" => Some(Builtin::History),
            _ => None,
        }
    }

    pub fn execute(
        &self,
        args: Vec<String>,
        rl: &Editor<ZhellHelper, DefaultHistory>,
    ) -> Result<String, Error> {
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
            Builtin::Pwd => match env::current_dir() {
                Ok(current_dir) => Ok(format!("{}", current_dir.display())),
                Err(e) => Err(e),
            },
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
            Builtin::History => {
                let history = rl.history();
                let history_string: String = history
                    .iter()
                    .enumerate()
                    .map(|(index, entry)| format!(" {:4}  {}", index + 1, entry))
                    .collect::<Vec<String>>()
                    .join("\n");

                Ok(history_string)
            }
        }
    }
}

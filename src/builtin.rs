use std::{
    env,
    io::{Error, ErrorKind},
};
#[allow(unused_imports)]
use rustyline::{
    Editor,
    history::{self, DefaultHistory},
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
        let joined_args = args.join(" ");
        match self {
            Builtin::Exit => std::process::exit(0),
            Builtin::Echo => Ok(format!("{}", joined_args)),
            Builtin::Type => {
                if Builtin::parse(joined_args.as_str()).is_some() {
                    Ok(format!("{} is a shell builtin", joined_args))
                } else if let Some(path) = find_in_path(joined_args.as_str()) {
                    Ok(format!("{} is {}", joined_args, path.display()))
                } else {
                    Ok(format!("{}: not found", joined_args))
                }
            }
            Builtin::Pwd => match env::current_dir() {
                Ok(current_dir) => Ok(format!("{}", current_dir.display())),
                Err(e) => Err(e),
            },
            Builtin::Cd => {
                let target_dir = if joined_args.is_empty() || joined_args == "~" {
                    match env::var("HOME") {
                        Ok(home) => home,
                        Err(_) => ".".to_string(),
                    }
                } else if joined_args.starts_with("~/") {
                    match env::var("HOME") {
                        Ok(home) => format!("{}{}", home, &joined_args[1..]),
                        Err(_) => joined_args.to_string(),
                    }
                } else if joined_args.starts_with("-") {
                    match env::var("OLDPWD") {
                        Ok(oldpwd) => oldpwd,
                        Err(_) => {
                            return Err(Error::new(ErrorKind::Other, "cd: OLDPWD not set"));
                        }
                    }
                } else {
                    joined_args.to_string()
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
                let history_list: Vec<String> = history
                    .iter()
                    .enumerate()
                    .map(|(index, entry)| format!(" {:4}  {}", index + 1, entry))
                    .collect();

                let display_limit = if let Some(first_arg) = args.first() {
                    first_arg.parse::<usize>().ok()
                } else {
                    None
                };

                let final_lines = if let Some(n) = display_limit {
                    let start_index = history_list.len().saturating_sub(n);
                    &history_list[start_index..]
                } else {
                    &history_list[..]
                };
                Ok(final_lines.join("\n"))
            }
        }
    }
}

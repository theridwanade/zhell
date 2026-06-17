use rustyline::{Context, completion::Completer};
use std::io::{Error, ErrorKind, Result, Write};
use std::{
    env,
    fs::{self, File, OpenOptions},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::builtin::Builtin;

pub fn find_in_path(program: &str) -> Option<PathBuf> {
    if let Ok(path_var) = env::var("PATH") {
        for mut directory in env::split_paths(&path_var) {
            directory.push(program);

            if let Ok(metadata) = fs::metadata(&directory) {
                if metadata.is_file() && metadata.permissions().mode() & 0o111 != 0 {
                    return Some(directory);
                }
            }
        }
    }
    None
}

#[derive(rustyline::Helper, rustyline::Highlighter, rustyline::Hinter, rustyline::Validator)]
pub struct ZhellHelper;

impl Completer for ZhellHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let mut candidates = Vec::new();

        let word_start = line[..pos].rfind(' ').map_or(0, |i| i + 1);
        let current_word = &line[word_start..pos];

        if word_start == 0 {
            for &cmd in &Builtin::COMMANDS {
                if cmd.starts_with(current_word) {
                    candidates.push(format!("{} ", cmd.to_string()));
                }
            }
        }

        if let Ok(path_var) = env::var("PATH") {
            for dir in env::split_paths(&path_var) {
                if let Ok(entries) = fs::read_dir(&dir) {
                    for entry in entries.flatten() {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_file() || file_type.is_symlink() {
                                if let Ok(file_name) = entry.file_name().into_string() {
                                    if file_name.starts_with(current_word) {
                                        candidates.push(format!("{} ", file_name));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        candidates.sort();
        candidates.dedup();

        Ok((word_start, candidates))
    }
}

pub struct RawArgs {
    pub actual_args: Vec<String>,
    pub output_file: Option<String>,
    pub error_output_file: Option<String>,
    pub to_append_output: bool,
}

pub fn process_raw_args(raw_args: &Vec<String>) -> RawArgs {
    let mut actual_args = Vec::new();
    let mut output_file = None;
    let mut error_output_file = None;
    let mut to_append_output = false;
    let mut i = 0;

    while i < raw_args.len() {
        match raw_args[i].as_str() {
            ">" | "1>" => {
                if i + 1 < raw_args.len() {
                    output_file = Some(raw_args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Syntax error: expected file after '{}'", raw_args[i]);
                    break;
                }
            }
            "2>" => {
                if i + 1 < raw_args.len() {
                    error_output_file = Some(raw_args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Syntax error: expected file after '{}'", raw_args[i]);
                    break;
                }
            }
            ">>" | "1>>" => {
                output_file = Some(raw_args[i + 1].clone());
                to_append_output = true;
                i += 2;
            }
            "2>>" => {
                error_output_file = Some(raw_args[i + 1].clone());
                to_append_output = true;
                i += 2;
            }
            _ => {
                actual_args.push(raw_args[i].clone());
                i += 1;
            }
        }
    }

    RawArgs {
        actual_args,
        output_file,
        error_output_file,
        to_append_output,
    }
}

pub fn execute_builtin_command(builtin: Builtin, args: RawArgs) {
    if let Some(ref file_path) = args.output_file {
        if args.to_append_output {
            let _ = OpenOptions::new().create(true).append(true).open(file_path);
        } else {
            let _ = File::create(file_path);
        }
    }
    if let Some(ref file_path) = args.error_output_file {
        if args.to_append_output {
            let _ = OpenOptions::new().create(true).append(true).open(file_path);
        } else {
            let _ = File::create(file_path);
        }
    }
    match builtin.execute(args.actual_args) {
        Ok(output) => {
            if let Some(file_path) = args.output_file {
                let file_result = if args.to_append_output {
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
            if let Some(file_path) = args.error_output_file {
                let file_result = if args.to_append_output {
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
}

pub fn execute_external_command(cmd: &str, args: RawArgs) -> Result<()> {
    if find_in_path(cmd).is_none() {
        return Err(Error::new(ErrorKind::NotFound, "command not found"));
    }
    let mut command = Command::new(cmd);
    command.args(args.actual_args);

    if let Some(file_path) = args.output_file {
        let file = if args.to_append_output {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&file_path)?
        } else {
            File::create(&file_path)?
        };
        command.stdout(Stdio::from(file));
    }

    if let Some(file_path) = args.error_output_file {
        let file = if args.to_append_output {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&file_path)?
        } else {
            File::create(&file_path)?
        };
        command.stderr(Stdio::from(file));
    }

    let mut child_process = command.spawn()?;
    child_process.wait()?;

    Ok(())
}

pub fn command_prompt() -> Result<String> {
    let current_working_dir = env::current_dir()?.display().to_string();
    let user = match env::var("USER") {
        Ok(user) => user,
        Err(_) => {
            eprintln!("Error: Could not get the current user");
            String::from("unknown")
        }
    };

    let hostname = match hostname::get() {
        Ok(hostname) => match hostname.into_string() {
            Ok(hostname_str) => hostname_str,
            Err(_) => {
                eprintln!("Error: Could not convert hostname to string");
                String::from("unknown")
            }
        },
        Err(_) => {
            eprintln!("Error: Could not get the hostname");
            String::from("unknown")
        }
    };
    let home = match env::var("HOME") {
        Ok(home) => home,
        Err(_) => {
            eprintln!("Error: Could not get the home directory");
            String::from("unknown")
        }
    };

    if current_working_dir.starts_with(&home) {
        let relative_path = current_working_dir.trim_start_matches(&home);
        return Ok(format!("{}@{}:~{}$ ", user, hostname, relative_path));
    }
    
    Ok(format!("{}@{}:{}$ ", user, hostname, current_working_dir))
}

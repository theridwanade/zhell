mod builtin;
mod lexer;
mod utils;

use std::io::ErrorKind;

use builtin::Builtin;
use rustyline::{
    CompletionType, Config, Editor, Result,
    error::ReadlineError,
    history::{DefaultHistory},
};

use crate::{
    lexer::tokenize,
    utils::{
        ZhellHelper, command_prompt, execute_builtin_command, execute_external_command,
        get_history_path, process_raw_args,
    },
};

fn main() -> Result<()> {
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();

    let mut rl = Editor::<ZhellHelper, DefaultHistory>::with_config(config)?;
    let history_path = get_history_path();
    if let Some(ref path) = history_path {
        if rl.load_history(path).is_err() {
            println!("No previous history found. Creating a new session.");
        }
    }
    let _ = rl.load_history("history.txt");
    let helper = ZhellHelper;
    rl.set_helper(Some(helper));
    loop {
        let command_prompt = command_prompt()?;
        let readline = rl.readline(&format!("{}", command_prompt));
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let input = tokenize(line.trim());

                if input.is_empty() {
                    continue;
                }
                if let Some(ref path) = history_path {
                    let _ = rl.append_history(path);
                }

                let (cmd, raw_args) = (input[0].as_str(), input[1..].to_vec());
                let processed_args = process_raw_args(&raw_args);

                if let Some(builtin) = Builtin::parse(cmd) {
                    execute_builtin_command(builtin, processed_args, &rl);
                } else {
                    if let Err(e) = execute_external_command(cmd, processed_args) {
                        if e.kind() == ErrorKind::NotFound {
                            eprintln!("{}: command not found", cmd);
                        } else {
                            eprintln!("Error: {:?}", e);
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

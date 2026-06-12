use std::{env, fs, os::unix::fs::PermissionsExt, path::PathBuf};

use rustyline::{Context, completion::Completer};

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

        // Only autocomplete built-ins if it's the first word on the line
        if word_start == 0 {
            // Iterate over the constant we added to your Builtin implementation
            for &cmd in &Builtin::COMMANDS {
                if cmd.starts_with(current_word) {
                    candidates.push(format!("{} ", cmd.to_string()));
                }
            }
        }

        Ok((word_start, candidates))
    }
}

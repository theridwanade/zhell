use std::{env, fs, os::unix::fs::PermissionsExt, path::PathBuf};

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

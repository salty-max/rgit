use std::{fs, path::PathBuf};

use crate::error::{ExitCode, ERROR_GIT};

pub fn init_repo(path: PathBuf) -> Result<String, (&'static str, ExitCode)> {
    let mut result = String::new();

    _ = fs::create_dir(".git");
    _ = fs::create_dir(".git/objects");
    _ = fs::create_dir(".git/refs");

    if fs::File::open(path.join(".git/HEAD")).is_err() {
        match fs::write(".git/HEAD", "ref: refs/heads/main\n") {
            Ok(_) => result.push_str("Initialized Git in current repository."),
            Err(_) => return Err(("Could not create Git HEAD pointer.", ERROR_GIT)),
        }
    } else {
        return Err(("Git repository already initialized.", ERROR_GIT));
    }

    Ok(result)
}

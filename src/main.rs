use std::{env, process::exit};

use rgit::{
    blob::read_blob,
    error::{ExitCode, ERROR_ARGUMENTS, ERROR_ENVIRONMENT, ERROR_UNKNOWN, SUCCESS},
    init::init_repo,
};

fn usage() -> &'static str {
    r#"
    usage: rgit [-v | --version] [-h | --help] <command> [<args>, ...]
        
    init                         Create an empty Git repository
    cat-file <object-id>         Print on object blob
    "#
}

fn main() {
    match run() {
        Ok(message) => println!("{message}"),
        Err((message, code)) => {
            eprintln!("{message}");
            exit(code);
        }
    }

    exit(SUCCESS);
}

fn run() -> Result<String, (&'static str, ExitCode)> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err((usage(), ERROR_ARGUMENTS));
    }

    let has_version = args.contains(&"-v".to_string()) || args.contains(&"--version".to_string());
    if has_version {
        return Ok(option_env!("CARGO_PKG_VERSION")
            .unwrap_or("Unknown")
            .to_string());
    }

    let has_help = args.contains(&"-h".to_string()) || args.contains(&"--help".to_string());
    if has_help {
        return Ok(usage().to_string());
    }

    let cwd = env::current_dir().map_err(|_| {
        (
            "Could not determine current working directory.",
            ERROR_ENVIRONMENT,
        )
    })?;

    match args[1].as_str() {
        "init" => init_repo(cwd),
        "cat-file" => {
            if args.len() != 3 {
                return Err((usage(), ERROR_ARGUMENTS));
            }

            read_blob(cwd, args[2].as_str())
        }
        _ => Err(("Unknown command", ERROR_UNKNOWN)),
    }
}

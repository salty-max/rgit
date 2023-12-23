use std::env;

use rgit::{blob::read_blob, init::init_repo};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "init" => init_repo(),
        "cat-file" => {
            read_blob(args[2].as_str());
        }
        _ => println!("Unknown command: {}", args[1]),
    }
}

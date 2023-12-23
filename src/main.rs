use std::{env, fs};

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git repository");
    } else {
        println!("Unknown command: {}", args[1]);
    }
}

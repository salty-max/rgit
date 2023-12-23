use std::{fs, io::Read};

use flate2::read::ZlibDecoder;

pub fn read_blob(hash: &str) {
    // The first two charactes of the hash are the folder name
    let folder_name = &hash[0..2];
    // The rest is the file name
    let file_name = &hash[2..];
    let path = format!(".git/objects/{folder_name}/{file_name}");

    let mut object = fs::File::open(&*path).unwrap();
    let mut content: Vec<u8> = vec![];
    let mut extracted_content = String::new();

    object.read_to_end(&mut content).unwrap();

    let mut decoder = ZlibDecoder::new(content.as_slice());

    match decoder.read_to_string(&mut extracted_content) {
        Ok(_) => {
            println!("{}", extracted_content);
        }
        Err(e) => {
            eprintln!("Error reading file {path}: {}", e);
        }
    }
}

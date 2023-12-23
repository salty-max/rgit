use std::{
    fs::read,
    io::{stdout, Read, Write},
    path::PathBuf,
    process::exit,
};

use flate2::read::ZlibDecoder;

use crate::error::{ExitCode, ERROR_ENVIRONMENT, ERROR_FILESYSTEM, SUCCESS};

pub fn read_blob(path: PathBuf, hash: &str) -> Result<String, (&'static str, ExitCode)> {
    // First two characters of the hash are the folder name, rest is the file name
    let (head, tail) = hash.split_at(2);
    let object_path = path.join(".git/objects").join(head).join(tail);

    if object_path.exists() {
        let file = read(object_path).map_err(|_| ("Could not read file.", ERROR_FILESYSTEM))?;

        let mut decoder = ZlibDecoder::new(file.as_slice());
        let mut buffer: Vec<u8> = Vec::new();

        decoder
            .read_to_end(&mut buffer)
            .map_err(|_| ("Could not read decoded blob to buffer.", ERROR_FILESYSTEM))?;

        if let Some(position) = &buffer.iter().position(|byte| byte == &0) {
            buffer.drain(..position + 1);
        }

        let mut io = stdout();
        io.write(&buffer)
            .map_err(|_| ("Could not print to stdout.", ERROR_ENVIRONMENT))?;
        io.flush()
            .map_err(|_| ("Could not flush write buffer to stdout.", ERROR_ENVIRONMENT))?;

        exit(SUCCESS)
    }

    Err(("Object does not exist.", ERROR_FILESYSTEM))
}

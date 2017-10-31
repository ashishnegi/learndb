use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::prelude::Write;
use std::io::prelude::Read;
use std::fs::OpenOptions;

// We give apis :
// create_key : creates a persisted key with value.
// get_value : get value for a key.
// put_value : change value for a key.
// key_exists : check if key exists.
// delete_key : deletes the key.

pub fn create_key(key : &str, value : &[u8]) -> Result<(), Error> {
    // How to atomically create a file if it does not exist ?
    // I don't want to do it in 2 calls.
    // Check if it exists and if not, then create. Race condition at `and`.
    // Currently we truncate the file.

    // key is our file name.
    let mut buffer = File::create(key)?;
    buffer.write_all(value)
}

pub fn get_value(key : &str) -> Result<Vec<u8>, Error> {
    let mut file = File::open(key)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

pub fn put_value(key : &str, value: &[u8]) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(key)?;
    file.write_all(value)
}

pub fn key_exists(key : &str) -> bool {
    fs::metadata(key).is_ok()
}

pub fn delete_key(key: &str) -> Result<(), Error> {
    fs::remove_file(key)
}

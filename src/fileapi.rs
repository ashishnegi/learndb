use std::fs::File;
use std::io::Error;
use std::io::prelude::Write;

pub fn create_key(key : &str, value : &[u8]) -> Result<(), Error> {
    // How to atomically create a file if it does not exist ?
    // I don't want to do it in 2 calls.
    // Check if it exists and if not, then create. Race condition at `and`.
    // Currently we truncate the file.

    // key is our file name.
    let mut buffer = File::create(key)?;
    buffer.write_all(value)
}


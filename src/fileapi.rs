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

pub trait Storage {
    fn get_value(&self, key : &str) -> Result<Vec<u8>, Error>;
    fn put_value(&self, key : &str, value: &[u8]) -> Result<(), Error>;
    fn key_exists(&self, key : &str) -> bool;
    fn delete_key(&self, key: &str) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct FileStorage {
    root_path: String
}

impl Storage for FileStorage {
    fn get_value(&self, key: &str) -> Result<Vec<u8>, Error> {
        let mut file = File::open(self.full_path(key))?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        Ok(contents)
    }

    fn put_value(&self, key: &str, value: &[u8]) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.full_path(key))?;
        file.write_all(value)
    }

    fn key_exists(&self, key: &str) -> bool {
        fs::metadata(self.full_path(key)).is_ok()
    }

    fn delete_key(&self, key: &str) -> Result<(), Error> {
        fs::remove_file(self.full_path(key))
    }
}

impl FileStorage {
    pub fn new(root_path : String) -> Self {
        FileStorage {
            root_path : root_path
        }
    }

    fn full_path(&self, key: &str) -> String {
        format!("{}/{}", self.root_path.as_str(), key)
    }
}

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
    fn put_value(&mut self, key : &str, value: &[u8]) -> Result<(), Error>;
    fn key_exists(&self, key : &str) -> bool;
    fn delete_key(&mut self, key: &str) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct FileStorage {
    root_path: String,
    new_key_dir: String
}

impl Storage for FileStorage {
    fn get_value(&self, key: &str) -> Result<Vec<u8>, Error> {
        let mut file = File::open(self.full_path(key))?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        Ok(contents)
    }

    fn put_value(&mut self, key: &str, value: &[u8]) -> Result<(), Error> {
        let tmp_file_path = self.full_path_new_key(key);
        {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(tmp_file_path.clone())?;
            file.write_all(value);
            file.flush();
        }

        // move file.
        fs::rename(tmp_file_path, self.full_path(key))
    }

    fn key_exists(&self, key: &str) -> bool {
        fs::metadata(self.full_path(key)).is_ok()
    }

    fn delete_key(&mut self, key: &str) -> Result<(), Error> {
        fs::remove_file(self.full_path(key))
    }
}

impl FileStorage {
    pub fn new(root_path : String, new_key_dir: String) -> Result<Self, Error> {
        let mut storage = FileStorage {
            root_path: root_path,
            new_key_dir: new_key_dir
        };

        storage.init();
        Ok(storage)
    }

    fn init(&mut self) -> Result<(), Error> {
        fs::create_dir(self.full_path_new_key(""))
    }

    fn full_path(&self, key: &str) -> String {
        format!("{}/{}", self.root_path.as_str(), key)
    }

    fn full_path_new_key(&self, key: &str) -> String {
        format!("{}/{}/{}", self.root_path.as_str(), self.new_key_dir.as_str(), key)
    }
}

impl Drop for FileStorage {
    fn drop(&mut self) {
        fs::remove_dir(self.full_path_new_key(""));
    }
}
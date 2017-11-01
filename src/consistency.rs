use std::collections::HashMap;
use std::sync::RwLock;
use fileapi::Storage;

// tries to give consistency layer of read/write locks over fileapi.

pub struct Consistency<'a> {
    locks: HashMap<String, RwLock<()>>,
    storage: &'a Storage
}

impl<'a> Consistency<'a> {
    pub fn new(storage: &'a Storage) -> Self {
        Consistency {
            locks : HashMap::new(),
            storage: storage
        }
    }
}

// impl<'a> Storage for Consistency<'a> {
//     fn get_value(&self, key : &str) -> Result<Vec<u8>, Error> {

//     }

//     fn put_value(&self, key : &str, value: &[u8]) -> Result<(), Error> {

//     }

//     fn key_exists(&self, key : &str) -> bool {

//     }

//     fn delete_key(&self, key: &str) -> Result<(), Error> {

//     }
// }
use std::sync::RwLock;
use fileapi::Storage;
use std::mem;
use std::ptr;
use std::io::Error;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Consistency layer of read/write locks over fileapi.

const CONCURRENCY : usize = 1000;

pub struct Consistency<'a> {
    locks: [RwLock<()>; CONCURRENCY],
    storage: &'a Storage
}

impl<'a> Consistency<'a> {
    pub fn new(storage: &'a Storage) -> Self {
        let array = unsafe {
            // Create an uninitialized array.
            let mut array: [RwLock<()>; CONCURRENCY] = mem::uninitialized();
            for (_, element) in array.iter_mut().enumerate() {
                // Overwrite `element` without running the destructor of the old value.
                // Since RwLock does not implement Copy, it is moved.
                ptr::write(element, RwLock::new(()))
            }
            array
        };

        Consistency {
            locks : array,
            storage: storage
        }
    }
}

impl<'a> Storage for Consistency<'a> {
    fn get_value(&self, key : &str) -> Result<Vec<u8>, Error> {
        let index = self.hash_key(key);
        let _ = self.locks[index].read().unwrap();
        self.storage.get_value(key)
    }

    fn put_value(&self, key : &str, value: &[u8]) -> Result<(), Error> {
        let index = self.hash_key(key);
        let _ = self.locks[index].write().unwrap();
        self.storage.put_value(key, value)
    }

    fn key_exists(&self, key : &str) -> bool {
        let index = self.hash_key(key);
        let _ = self.locks[index].read().unwrap();
        self.storage.key_exists(key)
    }

    fn delete_key(&self, key: &str) -> Result<(), Error> {
        let index = self.hash_key(key);
        let _ = self.locks[index].write().unwrap();
        self.storage.delete_key(key)
    }
}

impl<'a> Consistency<'a> {
    fn hash_key(&self, key: &str) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        (s.finish() as usize) % CONCURRENCY
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hash() {
        let ashish_hash;
        let ashish_negi_hash;
        let mut s = DefaultHasher::new();
        "ashish".hash(&mut s);
        ashish_hash = s.finish();

        let mut s = DefaultHasher::new();
        "ashish negi".hash(&mut s);
        ashish_negi_hash = s.finish();

        assert!(ashish_hash != ashish_negi_hash);
    }
}
use std::sync::RwLock;
use fileapi::Storage;
use std::mem;
use std::ptr;
use std::io::Error;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

// Consistency layer of read/write locks over fileapi.

const CONCURRENCY : usize = 1000;

pub struct Consistency {
    locks: [RwLock<()>; CONCURRENCY],
    storage: Arc<Storage+Sync+Send>
}

impl Consistency {
    pub fn new(storage: Arc<Storage+Sync+Send>) -> Self {
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

impl Storage for Consistency {
    fn get_value(&self, key : &str) -> Result<Vec<u8>, Error> {
        let index = self.hash_key(key);
        let r1 = self.locks[index].read().unwrap();
        self.storage.get_value(key)
    }

    fn put_value(&self, key : &str, value: &[u8]) -> Result<(), Error> {
        let index = self.hash_key(key);
        let r1 = self.locks[index].write().unwrap();
        self.storage.put_value(key, value)
    }

    fn key_exists(&self, key : &str) -> bool {
        let index = self.hash_key(key);
        let r1 = self.locks[index].read().unwrap();
        self.storage.key_exists(key)
    }

    fn delete_key(&self, key: &str) -> Result<(), Error> {
        let index = self.hash_key(key);
        let r1 = self.locks[index].write().unwrap();
        self.storage.delete_key(key)
    }
}

impl Consistency {
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

    use std::collections::HashMap;
    use std::io::ErrorKind;

    // #[derive(Debug)]
    // struct MemoryStorage {
    //     data: HashMap<String, Vec<u8>>
    // }

    // impl Storage for MemoryStorage
    // {
    //     fn get_value(&self, key : &str) -> Result<Vec<u8>, Error> {
    //         self.data.get(key).map_or(Err(Error::new(ErrorKind::NotFound, "Key does not exists")),
    //             |v| Ok(v.clone()))
    //     }

    //     fn put_value(&self, key : &str, value: &[u8]) -> Result<(), Error> {
    //         self.data.insert(key.to_string(), Vec::from(value));
    //         Ok(())
    //     }

    //     fn key_exists(&self, key : &str) -> bool {
    //         self.data.contains_key(key)
    //     }

    //     fn delete_key(&self, key: &str) -> Result<(), Error> {
    //         self.data.remove(key).map_or(Err(Error::new(ErrorKind::NotFound, "Key does not exists")),
    //             |v| Ok(()))
    //     }
    // }

    extern crate rand;
    use self::rand::Rng;
    use std::thread;
    use std::sync::Arc;
    use fileapi;
    use std::str;
    use std::thread::JoinHandle;

    #[test]
    fn multiple_threads_bombing_storage() {
        const parallels : usize = 10;
                          // MemoryStorage { data: HashMap::new() };
        let consistentArc = Arc::new(Consistency::new(Arc::new(fileapi::FileStorage::new(String::from("."), String::from("new_keys")).unwrap())));

        // create all keys.
        let mut keys = Vec::<String>::new();
        for k in 1..parallels {
            keys.push(k.to_string());
        }

        let zero = "0".as_bytes();
        // put default values in files.
        for key in keys.clone() {
            consistentArc.put_value(&key, zero).unwrap();
        }

        let consistency = consistentArc.clone();
        // test write threads
        let write_thread = Arc::new(move |to_write: Vec<u8>, keys: &mut Vec<String>| {
            let mut rng = rand::thread_rng();
            rng.shuffle(keys);

            println!("to_write: {:?} :: keys : {:?}", to_write, keys);

            for key in keys {
                let mut val = consistency.get_value(key).unwrap();
                val.append(&mut vec![13,10]);
                val.append(to_write.clone().as_mut());
                // println!("putting {:?} : {} {:?}", to_write, key, val);
                consistency.put_value(key, &val).unwrap();
                let mut read_back = consistency.get_value(key).unwrap();
                assert_eq!(val, read_back);
                // println!("done pu : {} {}", key, value);
            }
        });

        let mut handles = Vec::<JoinHandle<()>>::new();
        // start parallels threads..
        for t in 1..parallels {
            let write_thread = write_thread.clone();
            let mut keys = keys.clone();
            let handle = thread::spawn(move || {
                write_thread(t.to_string().into_bytes(), &mut keys);
            });
            handle.join().unwrap();
            // handles.push(handle);
        }

        // wait for all threads..
        for h in handles {
            h.join().unwrap();
        }

        thread::sleep_ms(1000);

        // check if all parallels keys have all numbers in any order.
        for key in keys.clone() {
            let val = consistentArc.get_value(&key).unwrap();
            let val_str = str::from_utf8(&val).unwrap();
            let mut vals : Vec<usize> = val_str.split("\r\n").map(|v| v.parse().unwrap()).collect();
            vals.sort();
            let expected : Vec<usize> = (0..parallels).collect();
            assert_eq!(vals, expected, "For file : {}" ,  key);
        }
    }
}
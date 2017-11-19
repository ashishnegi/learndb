extern crate rand;

use self::rand::Rng;
use std::sync::RwLock;
use std::thread;

use fileapi::Storage;
use std::mem;
use std::ptr;
use std::io::Error;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use transactional::{Transactional, WriteUnit};

// Consistency layer of read/write locks over fileapi.

const CONCURRENCY : usize = 1000;

pub struct Consistency {
    locks: [RwLock<()>; CONCURRENCY],
    storage: Arc<Storage+Sync+Send>,
    transaction_retires: u32,
    transaction_retry_wait_ms: u32
}

impl Consistency {
    pub fn new(storage: Arc<Storage+Sync+Send>,
               transaction_retires: u32,
               transaction_retry_wait_ms: u32 ) -> Self {
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
            storage: storage,
            transaction_retires: transaction_retires,
            transaction_retry_wait_ms: transaction_retry_wait_ms
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

impl Transactional for Consistency {
    fn write_multiple_keys(&self, units :&Vec<WriteUnit>) -> Result<(), Error> {
        let mut taken_locks = Vec::new();
        let mut tries = 0;
        let mut all_locked = false;

        let mut rng = rand::thread_rng();

        // TODO: add random sleep after N tries.
        while tries < self.transaction_retires
        {
            all_locked = true;
            for wu in units {
                let index = self.hash_key(&wu.key);
                let r1 = self.locks[index].try_write();
                if r1.is_err() {
                    // release all locks and retry.
                    taken_locks.clear();
                    all_locked = false;
                    break;
                }

                taken_locks.push(r1);
            }

            tries = tries + 1;

            if all_locked {
                break
            }

            // sleep for random ms.
            let sleep_for = self.transaction_retry_wait_ms * tries * ((rng.next_u32() % 10) + 1);
            println!("Sleeping for {}", sleep_for);
            thread::sleep_ms(sleep_for);
        }

        if all_locked {
            println!("Acquired all locks : writing values");
            self.storage.write_multiple_keys(units)
        } else {
            use std::io::ErrorKind;
            Err(Error::new(ErrorKind::Other, "Failed to acquire locks in multiple tries."))
        }
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

    // use std::collections::HashMap;
    // use std::io::ErrorKind;

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

    use std::thread;
    use fileapi;
    use std::str;
    use std::thread::JoinHandle;

    #[test]
    fn multiple_threads_bombing_storage() {
        const PARALLELS : usize = 10;
                          // MemoryStorage { data: HashMap::new() };
        let consistent_Arc = Arc::new(Consistency::new(

            Arc::new(fileapi::FileStorage::new(String::from("./multi"), String::from("multi_new_keys")).unwrap()),
            10,
            1));

        // create all keys.
        let mut keys = Vec::<String>::new();
        for k in 1..PARALLELS {
            keys.push(k.to_string());
        }

        let zero = "0".as_bytes();
        // put default values in files.
        for key in keys.clone() {
            consistent_Arc.put_value(&key, zero).unwrap();
        }

        let consistency = consistent_Arc.clone();
        // test write threads
        let write_thread = Arc::new(move |to_write: Vec<u8>, keys: &mut Vec<String>| {
            let mut rng = rand::thread_rng();
            rng.shuffle(keys);

            for key in keys {
                let mut val = consistency.get_value(key).unwrap();
                val.append(&mut vec![13,10]);
                val.append(to_write.clone().as_mut());
                // after i have read value, some other thread would have modified on file.
                // so only last writer wins.
                consistency.put_value(key, &val).unwrap();
            }
        });

        let mut handles = Vec::<JoinHandle<()>>::new();
        // start PARALLELS threads..
        for t in 1..PARALLELS {
            let write_thread = write_thread.clone();
            let mut keys = keys.clone();
            let handle = thread::spawn(move || {
                write_thread(t.to_string().into_bytes(), &mut keys);
            });

            handles.push(handle);
        }

        // wait for all threads..
        for h in handles {
            h.join().unwrap();
        }

        for key in keys.clone() {
            consistent_Arc.delete_key(&key).unwrap();
        }

        // check if all PARALLELS keys have all numbers in any order.
        // NOT POSSIBLE : see comment in write_thread.
        // for key in keys.clone() {
        //     let val = consistent_Arc.get_value(&key).unwrap();
        //     let val_str = str::from_utf8(&val).unwrap();
        //     let mut vals : Vec<usize> = val_str.split("\r\n").map(|v| v.parse().unwrap()).collect();
        //     vals.sort();
        //     let expected : Vec<usize> = (0..PARALLELS).collect();
        //     assert_eq!(vals, expected, "For file : {}" ,  key);
        // }
    }

    #[test]
    pub fn deadlock_test() {
        // 1000 threads, each doing 10 writes, 1 on each of 10 files.
        let mut handles = Vec::new();
        const PARALLELS :usize = 1000;
        const NUM_KEYS :usize = 10;

        let consistent = Arc::new(Consistency::new(
            Arc::new(fileapi::FileStorage::new(String::from("./deadlock"), String::from("deadlock_new_keys")).unwrap()),
            10,
            1));

        let consistency = consistent.clone();

        let write_thread = Arc::new(move || {
            let mut keys :Vec<usize> = (1..NUM_KEYS).collect();
            let mut rng = rand::thread_rng();
            rng.shuffle(&mut keys);

            for k in keys.iter() {
                let r = rng.next_u32() % 10;
                let key = &k.to_string();
                if r < 10 {
                    consistency.put_value(key, &[1,2,3]).unwrap(); // should never fail.
                } else if r < 95 {
                    consistency.get_value(key); // it can fail if key does not exists.
                } else {
                    consistency.delete_key(key); // will fail if key does not exists.
                }
            }
        });

        for _ in 1..PARALLELS {
            let write_thread = write_thread.clone();
            let handle = thread::spawn(move || write_thread());
            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }

        for key in (1..NUM_KEYS).map(|v| v.to_string()) {
            if consistent.key_exists(&key) {
                consistent.delete_key(&key).unwrap();
            }
        }
    }

    #[test]
    pub fn transactional() {
        // each thread writes in all files its own unique value.
        // at the end of all tests fired, each file should have only same value.
        let mut handles = Vec::new();
        const PARALLELS :usize = 10;
        const NUM_KEYS :usize = 10;

        let consistent = Arc::new(Consistency::new(
            Arc::new(fileapi::FileStorage::new(String::from("./transactional"), String::from("transactional_new_keys")).unwrap()),
            10,
            1));

        let consistency = consistent.clone();

        let write_thread = Arc::new(move |thread_id: usize| {
            let mut keys :Vec<usize> = (1..NUM_KEYS).collect();
            let mut rng = rand::thread_rng();
            rng.shuffle(&mut keys);
            let mut write_units = Vec::new();
            for k in keys {
                write_units.push(WriteUnit{
                    key: k.to_string(),
                    value: thread_id.to_string().into_bytes()
                })
            }

            use std::error::Error;
            match consistency.write_multiple_keys(&write_units) {
                Err(err) => assert!(err.description().contains("Failed to acquire locks in multiple tries")),
                Ok(_) => ()
            }
        });

        for i in 1..PARALLELS {
            let write_thread = write_thread.clone();
            let handle = thread::spawn(move || write_thread(i));
            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }

        let expected_value = consistent.get_value("1").unwrap();
        for key in (2..NUM_KEYS).map(|v| v.to_string()) {
            assert_eq!(expected_value, consistent.get_value(&key).unwrap(), "All files don't have same value");
        }

        for key in (1..NUM_KEYS).map(|v| v.to_string()) {
            if consistent.key_exists(&key) {
                consistent.delete_key(&key).unwrap();
            }
        }
    }
}
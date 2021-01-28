use crate::api::{KVError, KVStore};
use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;
use std::time::SystemTime;
use std::fmt::Debug;
use std::sync::{Mutex, Arc};

///This is a simple wrapper around HashMap<K,V> that adds thread safety and optional expiration
#[derive(Default,Debug,Clone)]
pub struct HashCache<K, V> {
    pub cache: Arc<Mutex<HashMap<K, (Option<SystemTime>, V)>>>,
}

//Implements that trait defined in api.rs
impl<K: Clone + Eq + Hash + Debug, V: Clone + Debug> KVStore<K, V> for HashCache<K, V> {
    ///Gets a lock on the underlying HashMap and inserts a value into it
    fn put(&mut self, k: K, v: V, expiration: Option<Duration>) -> Result<(), KVError> {
        match expiration {
            Some(exp) => self.cache.lock().unwrap().insert(k, (Some(SystemTime::now() + exp), v)),
            None => self.cache.lock().unwrap().insert(k, (None, v)),
        };
        Ok(())
    }

    ///Gets a lock on the underlying HashMap and retrieves a value from it
    ///In this cache implementation, expired records are deleted at read time,
    /// hence the relative complexity of this implementation
    fn get(&mut self, k: K) -> Option<V> {
        let mut cache = self.cache.lock().unwrap();
        match cache.get(&k) {
            None => {
                //No value found
                None
            }
            Some((t, v)) => {
                match t {
                    None => {
                        //No expiration time, so key always valid
                        Some(v.clone())
                    },
                    Some(t) => {
                        if t > &SystemTime::now() {
                            //key still valid
                            Some(v.clone())
                        } else {
                            //key expired
                            cache.remove(&k);
                            None
                        }
                    }
                }
            }
        }
    }

    ///gets a lock on the underlying HashMap and deletes the specified entry
    fn delete(&mut self, k: K) -> Result<(), KVError> {
        self.cache.lock().unwrap().remove(&k);
        Ok(())
    }
}

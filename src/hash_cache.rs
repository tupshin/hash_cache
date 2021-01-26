use crate::api::{KVError, KVStore};
use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;
use std::time::SystemTime;
use std::fmt::Debug;
use std::sync::{Mutex, Arc};

#[derive(Default,Debug,Clone)]
pub struct HashCache<K, V> {
    pub cache: Arc<Mutex<HashMap<K, (Option<SystemTime>, V)>>>,
}

unsafe impl<K,V> Send for HashCache<K,V> {}
unsafe impl<K,V> Sync for HashCache<K,V> {}

impl<K: Clone + Eq + Hash + Debug, V: Clone + Debug> KVStore<K, V> for HashCache<K, V> {
    fn put(&mut self, k: K, v: V, expiration: Option<Duration>) -> Result<(), KVError> {
        match expiration {
            Some(exp) => self.cache.lock().unwrap().insert(k, (Some(SystemTime::now() + exp), v)),
            None => self.cache.lock().unwrap().insert(k, (None, v)),
        };
        Ok(())
    }

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

    fn delete(&mut self, k: K) -> Result<(), KVError> {
        self.cache.lock().unwrap().remove(&k);
        Ok(())
    }
}

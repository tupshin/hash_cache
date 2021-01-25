use crate::api::{KVError, KVStore};
use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;
use std::time::SystemTime;

#[derive(Default,Debug)]
pub struct HashCache<K, V> {
    cache: HashMap<K, (Option<SystemTime>, V)>,
}

impl<K: Clone + Eq + Hash, V: Clone> KVStore<K, V> for HashCache<K, V> {
    fn put(&mut self, k: K, v: V, expiration: Option<Duration>) -> Result<(), KVError> {
        match expiration {
            Some(exp) => self.cache.insert(k, (Some(SystemTime::now() + exp), v)),
            None => self.cache.insert(k, (None, v)),
        };
        Ok(())
    }

    fn get(&mut self, k: K) -> Option<V> {
        match self.cache.get(&k) {
            None => None,
            Some((t, v)) => {
                match t {
                    None => None,
                    Some(t) => {
                        if t > &SystemTime::now() {
                            //key still valid
                            Some(v.clone())
                        } else {
                            //key expired
                            self.cache.remove(&k);
                            None
                        }
                    }
                }
            }
        }
    }

    fn delete(&mut self, k: K) -> Result<(), KVError> {
        self.cache.remove(&k);
        Ok(())
    }
}

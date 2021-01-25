use std::hash::Hash;
use std::time::Duration;

///The trait-based approach to an API allows me to compare multiple implementation approaches
/// more easily, and is generally a good practice when designing APIs
pub trait KVStore<K: Clone + Eq + Hash, V: Clone> {
    fn put(&mut self, k: K, v: V, expiration: Option<Duration>) -> Result<(), KVError>;

    ///The API of ```get()``` assumes that we don't need to expose any internal errors up the call chain
    /// and simple presense of absence of value is sufficient.
    /// Even though get() is a read operation, it can potentially mutate itself by performing a deletion, hence the $mut
    ///Note that get must check for expired data and delete K/V pairs that have expired
    fn get(&mut self, k: K) -> Option<V>;

    fn delete(&mut self, k: K) -> Result<(), KVError>;
}

#[derive(Debug)]
pub enum KVError {}

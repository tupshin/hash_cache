pub mod api;
pub mod hash_cache;
#[cfg(test)]
mod tests {
    use crate::api::KVStore;
    use crate::hash_cache::HashCache;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_put() {
        let mut cache = HashCache::default();
        cache.put("a", "b", None).unwrap();
    }

    #[test]
    fn test_get() {
        let mut cache = HashCache::default();
        cache.put("a", "b", None).unwrap();
        let result = cache.get("a").unwrap();
        assert_eq!(result, "b");
    }

    #[test]
    fn test_delete() {
        let mut cache = HashCache::default();
        cache.put("a", "b", None).unwrap();
        let result = cache.get("a").unwrap();
        assert_eq!(result, "b");
        cache.delete("a").unwrap();
        let deleted_val = cache.get("a");
        assert_eq!(deleted_val, None)
    }

    #[test]
    fn test_expiring_put() {
        let mut cache1 = HashCache::default();
        //This clones the internal Arc
        //let mut cache2 = cache1.clone();
        let duration = Duration::new(1, 0);
        cache1.put("a", "b", Some(duration)).unwrap();

        assert_eq!(Some("b"), cache1.get("a"));
        thread::sleep(duration);
        assert_eq!(None, cache1.get("a"));
    }

    #[test]
    fn multi_threaded_put_get() {
        let cache = HashCache::default().cache;
        let cache1 = cache.clone();
        let cache2 = cache1.clone();

        let thread_one = thread::spawn(move || {
            cache1.lock().unwrap().insert("A", (None, "a"));
        });
        let thread_two = thread::spawn(move || {
            cache2.lock().unwrap().insert("B", (None, "b"));
        });

        thread_one.join().unwrap();
        thread_two.join().unwrap();

        thread::spawn(move || {
            assert_eq!(cache.lock().unwrap().get("A").unwrap().1, "a");
            assert_eq!(cache.lock().unwrap().get("B").unwrap().1, "b");
        });
    }

    #[test]
    fn bulk_put_get() {
        let cache = HashCache::default().cache;
        let cache1 = cache.clone();
        let cache2 = cache1.clone();

        let thread_one = thread::spawn(move || {
            for i in 1..=1000 {
                cache1.lock().unwrap().insert(i, (None, format!("v:{}", i)));
            }
        });
        thread_one.join().unwrap();

        let thread_two = thread::spawn(move || {
            assert_eq!(cache2.lock().unwrap().get(&75).unwrap().1, format!("v:{}", 75));
            assert_eq!(cache2.lock().unwrap().get(&352).unwrap().1, format!("v:{}", 352));
        });
        thread_two.join().unwrap();
    }

    #[test]
    fn ten_million_load() {
        let cache = HashCache::default().cache;
        let cache1 = cache.clone();
        let cache2 = cache1.clone();

        let thread_one = thread::spawn(move || {
            for i in 1..=1_000_000 {
                cache1.lock().unwrap().insert(i, (None, format!("v:{}", i)));
            }
        });
        thread_one.join().unwrap();

        let thread_two = thread::spawn(move || {
            assert_eq!(cache2.lock().unwrap().get(&75).unwrap().1, format!("v:{}", 75));
            assert_eq!(cache2.lock().unwrap().get(&352).unwrap().1, format!("v:{}", 352));
        });
        thread_two.join().unwrap();
    }
}
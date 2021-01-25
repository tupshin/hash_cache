pub mod api;
pub mod hash_cache;
#[cfg(test)]
mod tests {
    use crate::api::KVStore;
    use crate::hash_cache::HashCache;
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
        dbg!(&cache);
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
        let mut cache = HashCache::default();
        cache.put("a", "b", Some(Duration::new(1, 0))).unwrap();

    }
}

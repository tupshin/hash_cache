use criterion::{criterion_group, criterion_main, Criterion};
use hash_cache::api::KVStore;
use hash_cache::hash_cache::HashCache;
use rand::Rng;
use std::thread;
use std::time::{Duration, SystemTime};

///inserts 10 million records and makes sure that we can retrieve one
fn insert_benchmark(c: &mut Criterion) {
    let mut cache = HashCache::default();
    bulk_insert(&mut cache, 10_000_000);
    let mut rng = rand::thread_rng();
    c.bench_function("read_from_10M", |b| {
        b.iter(|| read_one(&mut cache, rng.gen_range(1..=10_000_000)))
    });
}

fn insert_expiring_benchmark(c: &mut Criterion) {
    let mut cache = HashCache::default();
    let duration = Duration::new(1, 0);
    bulk_insert_expiring(&mut cache, 10_000_000, duration);
    let mut rng = rand::thread_rng();
    c.bench_function("read_from_expiring_10M", |b| {
        b.iter(|| read_expiring_one(&mut cache, rng.gen_range(1..=10_000_000)))
    });
}

fn read_one(cache: &mut HashCache<String, String>, num: usize) {
    let val = cache.get(format!("k:{}", num));
    assert_eq!(val, Some(format!("v:{}", num)));
}

fn read_expiring_one(cache: &mut HashCache<String, String>, num: usize) {
    let val = cache.get(format!("k:{}", num));
    // assert_eq!(val, Some(format!("v:{}",num)));
}

fn bulk_insert(cache: &mut HashCache<String, String>, count: usize) {
    for i in 1..=count {
        cache.put(format!("k:{}", i), format!("v:{}", i), None);
    }
}

fn bulk_insert_expiring(cache: &mut HashCache<String, String>, count: usize, expiration: Duration) {
    for i in 1..=count {
        cache.put(format!("k:{}", i), format!("v:{}", i), Some(expiration));
    }
}

criterion_group!(benches, insert_benchmark, insert_expiring_benchmark);
criterion_main!(benches);

```cargo new --lib kv_store```

Assess requirements:

* Project
  * in memory caching library
  * caller facing api
  * functional verification tests
  * document performance and memory usage
  * thread safe  
* Functional (translated to api fn calls)
  * ```pub fn put(&self, k:K,v:V,expiration: Option<Duration>) -> Result<(),Error>```
  * ```pub fn get(k:K) -> Option<V>```
  * ```pub fn delete(k:K) -> Result<(),Error>```
    
* Demonstrable Performance
  * Retrieve keys where 95th percentile latency is less than 1 second
  * Retrieve keys where 99th percentile latency 
  * Handle 10M k/v pairs
    

Thoughts on requirements and initial approach:
* Unspecified what the size and type of the keys and values should be. 
    * Will require keys to implement ```Ord```
    * Will require keys and values to implement ```Clone```
    * Won't constrain size of either for now, even though that allows for easy denial of service attack
* Concurrency and load testing are not mentioned, except in terms of 95th and 99th percentile read latencies
    * For the purposes of this exercise, I will assume little contention and have one writer thread, and only two threads of read concurrency, each looping with a couple milleseconds pause between calls
    
* Testing
  * Implement test data loader using ```put()``` with no timeout and loading 10M records.
  * Size data to fit in RAM such that 10M k/v pairs should occupy 1-2GB of memory
  * Implement 3 threaded client that demonstrates a single writer and two readers  and measure the latency of each operation
    * Open question at this point, best way to benchmark rust code like this
    
    
Additional implementation thoughts before coding:

* I'm assuming using stdlib data structures is not an issue, and it's tempting to see how far you can get by using
stock ```HashMap<K,(Option<ExpTime>,V)>``` behind a Mutex
* Expiring data introduces lots of new performance complexities. For time reasons I'm going to go with the very simple
read-based expiry approach where no data gets deleted due to expiring until a read attempt is made for that key, at which point a delete will be performed. In a production system, an additional cron reaper would be needed to purge expired keys.
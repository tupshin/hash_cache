Given the limited time for this exercise, I opted to go with a very simple approach of reusing HashMap and putting it behind
a thread safe wrapper, and using a tuple of (expire_time,Value) to track expiring data

Issues with this approach include:

* Poor performance under high contention
* Data must fit in single machine's RAM (not distributed)
* Not robust against denial-of-service attacks

Limitations of the current implementation (with more time would implement) include:
* Partial test suite
* Performance is benchmarked using criterion but 95th and 99th percentile targets not formally measured
* No significant performance measurements under contention
* Uses unwrap all over the place instead of decent error handling
* Uses a trait as an abstraction around multiple possible implementations but only implements one
* Only deletes at read time. A background garbage collector could be implemented to improve that

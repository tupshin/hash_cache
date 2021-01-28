The accompanying code largely fulfills the goals of the thread-safe expiring cache that was specified.

Please see the NOTES.md file to see my initial thinking.
Note that the approach I outline there is the approach I ended up using.

Design goals:
Write a thread safe caching implementation that has good 95th and 99th percentile latencies and also supports expiring keys

**Approach:**

* Design a trait with the necesaary functions
* Take the stdlib HashMap and wrap it behind an Arc<Mutex> for thread safety.
* Implement the above trait for my custom type

  * Support generic K/V types, as long as they implement the same traits that HashMap requires, such as Eq and Hash.
  * Instead of just storing the Value in the HashMap, store a tuple of (expiration_time, Value), so that values
that already expired can be ignored at retrieval time and lazily deleted

see the ISSUES.md file for a description of the limitations of this approach and partial implementation

**Running:**

To run the tests, use ```cargo test```
To run the performance benchmarks, use cargo to install criterion
```cargo intall cargo-criterion```
and then run ```cargo criterion```
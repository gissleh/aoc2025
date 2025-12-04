# AOC 2025

My annual use case for the Rust programming language, trying to get the lowest runtime I can get even at the cost of readability.

## Running
Set the `AOC_SESSION` environment variable to be able to download the input files, then run this command:

```sh
cargo run --release --bin aoc2025 -- command day
```

* `command`: The command to run
  * `run`, `<blank>`: Just get the answer
  * `bench`: Benchmark with warm cache
  * `bench_once`: Benchmark with cold cache
* `day`: Which day to run
  * `<blank>`: Current day
  * `0`: All days
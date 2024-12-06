# Advent of Code 2024

Hi, I'm Alexander Lowry and these are my solutions to Avent of Code 2024.
I used Rust because I like it!

I benchmark my solutions with `hyperfine` and the `--release` flag.

```hyperfine 'cargo run --bin day_{n} --release' --warmup 2```

## Day 1

Created a utils modules to avoid boilerplate.
We're always going to be parsing a bunch of from files, so I've added a bunch of file and Vec utilities.

### Benchmark

#### Apple M2 Pro, 16 GB
```
Benchmark 1: cargo run --bin day_1 --release
Time (mean ± σ):     173.3 ms ±   5.7 ms    [User: 32.6 ms, System: 13.8 ms]
Range (min … max):   163.4 ms … 191.4 ms    15 runs
```

## Day 2


### Benchmark

#### Apple M2 Pro, 16 GB
```
Benchmark 1: cargo run --bin day_2 --release
  Time (mean ± σ):     175.3 ms ±   7.4 ms    [User: 32.4 ms, System: 14.1 ms]
  Range (min … max):   166.6 ms … 193.7 ms    15 runs
```

## Day 3

### Benchmark

#### Apple M2 Pro, 16 GB
```
Benchmark 1: cargo run --bin day_3 --release
  Time (mean ± σ):     193.0 ms ±   1.9 ms    [User: 35.8 ms, System: 15.6 ms]
  Range (min … max):   190.7 ms … 196.2 ms    15 runs
```

## Day 4

### Benchmark

#### Apple M2 Pro, 16 GB
```
Benchmark 1: cargo run --bin day_4 --release
  Time (mean ± σ):     174.7 ms ±   3.0 ms    [User: 33.7 ms, System: 15.3 ms]
  Range (min … max):   171.3 ms … 182.9 ms    16 runs
```

## Day 5

### Benchmark

#### Apple M2 Pro, 16 GB
```
Benchmark 1: cargo run --bin day_5 --release
  Time (mean ± σ):     182.1 ms ±   3.0 ms    [User: 41.9 ms, System: 14.2 ms]
  Range (min … max):   176.3 ms … 186.6 ms    16 runs
```

## Day 6

This problem really had me pulling my hair out on part 2.
My original solution for part 1 was copying the whole map over and over to record the history,
which ended up being a really slow and bad idea.
Instead, I switched to storing a history within a single mutable map.
The history is now a fixed size allocated once.


### Benchmark

#### Apple M2 Pro, 16 GB
```
Benchmark 1: cargo run --bin day_6 --release
  Time (mean ± σ):     409.3 ms ±   5.6 ms    [User: 235.2 ms, System: 14.4 ms]
  Range (min … max):   398.7 ms … 420.2 ms    10 runs
```

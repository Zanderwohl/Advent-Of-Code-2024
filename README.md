# Advent of Code 2024

Hi, I'm Alexander Lowry and these are my solutions to [Advent of Code 2024](https://adventofcode.com/2024/).
I used Rust because I like it!

I benchmark my solutions with the `--release` flag in two ways:
[hyperfine](https://github.com/sharkdp/hyperfine) (which means a minimum 150ms for OS stuff),
and internally using `std::time::Instant;`, because hyperfine doesn't seem to measure anything below 150ms
(`fn main() {}` takes 151ms on my machine!).
The downside, of course, is that I am just marking two times, ignoring any switching the CPU is doing.
Times depend on if the computer is doing anything else!

I haven't modified my `cargo.toml` yet, because I'm okay with the defaults for now :).

```hyperfine 'cargo run --bin day_{n:02} --release' --warmup 2```

## Day 1

[Problem Text](https://adventofcode.com/2024/day/1)

Created a utils modules to avoid boilerplate.
We're always going to be parsing a bunch of from files, so I've added a bunch of file and Vec utilities.

### Benchmark

#### Apple M3 Pro, 18 GB
`Completed in: 797.25µs`
```
Benchmark 1: cargo run --bin day_01 --release
  Time (mean ± σ):     163.6 ms ±  10.7 ms    [User: 35.4 ms, System: 12.6 ms]
  Range (min … max):   142.7 ms … 178.5 ms    19 runs
```

#### Apple M2 Pro, 16 GB
`Completed in: 566.667µs`
```
Benchmark 1: cargo run --bin day_01 --release
Time (mean ± σ):     173.3 ms ±   5.7 ms    [User: 32.6 ms, System: 13.8 ms]
Range (min … max):   163.4 ms … 191.4 ms    15 runs
```

## Day 2
[Problem Text](https://adventofcode.com/2024/day/2)

### Benchmark

#### Apple M3 Pro, 18 GB
`Completed in: 1.054959ms`
```
Benchmark 1: cargo run --bin day_02 --release
  Time (mean ± σ):     166.4 ms ±  10.0 ms    [User: 36.4 ms, System: 13.9 ms]
  Range (min … max):   155.3 ms … 197.4 ms    19 runs
```

#### Apple M2 Pro, 16 GB
`Completed in: 841.292µs`
```
Benchmark 1: cargo run --bin day_02 --release
  Time (mean ± σ):     175.3 ms ±   7.4 ms    [User: 32.4 ms, System: 14.1 ms]
  Range (min … max):   166.6 ms … 193.7 ms    15 runs
```

## Day 3
[Problem Text](https://adventofcode.com/2024/day/3)

### Benchmark

#### Apple M3 Pro, 18 GB
`Completed in: 1.484083ms`
```
Benchmark 1: cargo run --bin day_03 --release
  Time (mean ± σ):     172.7 ms ±   2.0 ms    [User: 35.2 ms, System: 12.1 ms]
  Range (min … max):   167.9 ms … 174.8 ms    17 runs
```

#### Apple M2 Pro, 16 GB
`Completed in: 1.491125ms`
```
Benchmark 1: cargo run --bin day_03 --release
  Time (mean ± σ):     193.0 ms ±   1.9 ms    [User: 35.8 ms, System: 15.6 ms]
  Range (min … max):   190.7 ms … 196.2 ms    15 runs
```

## Day 4
[Problem Text](https://adventofcode.com/2024/day/4)

### Benchmark

#### Apple M3 Pro, 18 GB
`Completed in: 1.139375ms`
```
Benchmark 1: cargo run --bin day_04 --release
  Time (mean ± σ):     152.9 ms ±   3.5 ms    [User: 32.9 ms, System: 9.9 ms]
  Range (min … max):   142.6 ms … 157.2 ms    19 runs
```

#### Apple M2 Pro, 16 GB
`Completed in: 880.208µs`
```
Benchmark 1: cargo run --bin day_04 --release
  Time (mean ± σ):     174.7 ms ±   3.0 ms    [User: 33.7 ms, System: 15.3 ms]
  Range (min … max):   171.3 ms … 182.9 ms    16 runs
```

## Day 5
[Problem Text](https://adventofcode.com/2024/day/5)

### Benchmark

#### Apple M3 Pro, 18 GB
`Completed in: 15.673542ms`
```
Benchmark 1: cargo run --bin day_05 --release
  Time (mean ± σ):     160.1 ms ±   2.6 ms    [User: 40.8 ms, System: 8.5 ms]
  Range (min … max):   155.5 ms … 164.7 ms    18 runs

```

#### Apple M2 Pro, 16 GB
`Completed in: 8.766542ms`
```
Benchmark 1: cargo run --bin day_05 --release
  Time (mean ± σ):     182.1 ms ±   3.0 ms    [User: 41.9 ms, System: 14.2 ms]
  Range (min … max):   176.3 ms … 186.6 ms    16 runs
```

## Day 6
[Problem Text](https://adventofcode.com/2024/day/6)

This problem really had me pulling my hair out on part 2.
My original solution for part 1 was copying the whole map over and over to record the history,
which ended up being a really slow and bad idea.
Instead, I switched to storing a history within a single mutable map.
The history is now a fixed size allocated once.

### Benchmark

#### Apple M3 Pro, 18 GB
`Completed in: 182.549875ms`
```
Benchmark 1: cargo run --bin day_06 --release
  Time (mean ± σ):     353.0 ms ±   1.2 ms    [User: 196.1 ms, System: 7.2 ms]
  Range (min … max):   351.1 ms … 354.7 ms    10 runs
```

#### Apple M2 Pro, 16 GB
`Completed in: 196.226708ms`
```
Benchmark 1: cargo run --bin day_06 --release
  Time (mean ± σ):     409.3 ms ±   5.6 ms    [User: 235.2 ms, System: 14.4 ms]
  Range (min … max):   398.7 ms … 420.2 ms    10 runs
```

## Day 7
[Problem Text](https://adventofcode.com/2024/day/7)

Oof, this was a tough one for run times.
At first, I was just checking every combination then quitting as the accumulated total got too big.
When I saw that people were getting smaller times than me in Rust,
I returned to re-implement the solver using recursion,
which meant that I could quit a lot quicker when I saw the accumulator was getting too big.
Thus, I reduced my solution to under 1 second.

### Benchmark

#### Apple M3 Pro, 18 GB

##### Without Recursion
```
Benchmark 1: cargo run --bin day_07 --release
  Time (mean ± σ):      1.861 s ±  0.024 s    [User: 1.694 s, System: 0.013 s]
  Range (min … max):    1.831 s …  1.902 s    10 runs
```

##### With Recursion
`Completed in: 483.940916ms`
```
Benchmark 1: cargo run --bin day_07 --release
  Time (mean ± σ):     654.4 ms ±   4.4 ms    [User: 496.2 ms, System: 8.4 ms]
  Range (min … max):   649.1 ms … 660.6 ms    10 runs
```

#### Apple M2 Pro, 16 GB

##### With Recursion
`Completed in: 529.033958ms`
```
Benchmark 1: cargo run --bin day_07 --release
  Time (mean ± σ):     718.5 ms ±   3.2 ms    [User: 539.7 ms, System: 23.0 ms]
  Range (min … max):   714.1 ms … 724.0 ms    10 runs
```

## Day 8
[Problem Text](https://adventofcode.com/2024/day/8)

### Benchmark

#### Apple M3 Pro, 18 GB
`Completed in: 316.042µs`
```
Benchmark 1: cargo run --bin day_08 --release
  Time (mean ± σ):     153.9 ms ±   2.4 ms    [User: 32.9 ms, System: 10.3 ms]
  Range (min … max):   150.1 ms … 159.4 ms    19 runs
```

#### Apple M2 Pro, 16 GB
`Completed in: 369.625µs`
```
Benchmark 1: cargo run --bin day_08 --release
  Time (mean ± σ):     172.2 ms ±   3.7 ms    [User: 31.9 ms, System: 14.3 ms]
  Range (min … max):   167.0 ms … 182.7 ms    16 runs

```

## Day 9
[Problem Text](https://adventofcode.com/2024/day/9)

### Benchmark

#### Apple M3 Pro, 18 GB
`Completed in: 167.809167ms`
```
Benchmark 1: cargo run --bin day_09 --release
  Time (mean ± σ):     343.3 ms ±   4.0 ms    [User: 177.0 ms, System: 18.1 ms]
  Range (min … max):   335.9 ms … 349.0 ms    10 runs
```

#### Apple M2 Pro, 16 GB
`Completed in: 187.164084ms`
```
Benchmark 1: cargo run --bin day_09 --release
  Time (mean ± σ):     381.2 ms ±   3.5 ms    [User: 204.7 ms, System: 22.3 ms]
  Range (min … max):   376.3 ms … 389.2 ms    10 runs
```

## Day 10
[Problem Text](https://adventofcode.com/2024/day/10)

### Benchmark

#### Apple M3 Pro, 18 GB
`Completed in: 658.458µs`
```
Benchmark 1: cargo run --bin day_10 --release
  Time (mean ± σ):     151.5 ms ±   0.9 ms    [User: 33.1 ms, System: 9.2 ms]
  Range (min … max):   149.9 ms … 152.8 ms    19 runs
```

#### Apple M2 Pro, 16 GB
`Completed in: 673.125µs`
```
Benchmark 1: cargo run --release --bin day_10
  Time (mean ± σ):     174.5 ms ±   3.3 ms    [User: 32.9 ms, System: 15.0 ms]
  Range (min … max):   166.9 ms … 179.5 ms    17 runs

```

## Day 11
[Problem Text](https://adventofcode.com/2024/day/11)

Oddly performed batter on the M2.
Probably got more consistent processor scheduling.

### Benchmark

#### Apple M3 Pro, 18 GB
`Completed in: 54.181917ms`
```
Benchmark 1: cargo run --release --bin day_11
  Time (mean ± σ):     157.4 ms ±   1.2 ms    [User: 39.4 ms, System: 8.4 ms]
  Range (min … max):   155.3 ms … 159.2 ms    18 runs
```

#### Apple M2 Pro, 16 GB
`Completed in: 7.541333ms`
```
Benchmark 1: cargo run --release --bin day_11
  Time (mean ± σ):     181.5 ms ±   6.4 ms    [User: 37.4 ms, System: 15.1 ms]
  Range (min … max):   169.5 ms … 189.7 ms    17 runs
```

## Day 12
[Problem Text](https://adventofcode.com/2024/day/12)

Again, I'm not sure why this completes 12x faster on the M2.
Perhaps it's just the coincidence of fewer background tasks or less context switching.

### Benchmark

#### Apple M3 Pro, 18 GB
`Completed in: 12.984166ms`
```
Benchmark 1: cargo run --release --bin day_12
  Time (mean ± σ):     154.7 ms ±   1.8 ms    [User: 33.7 ms, System: 9.6 ms]
  Range (min … max):   152.2 ms … 159.4 ms    19 runs
```

#### Apple M2 Pro, 16 GB
`Completed in: 1.01075ms`
```
Benchmark 1: cargo run --release --bin day_12
  Time (mean ± σ):     173.9 ms ±   5.5 ms    [User: 29.4 ms, System: 14.7 ms]
  Range (min … max):   166.5 ms … 185.4 ms    17 runs
```

## Day 12
[Problem Text](https://adventofcode.com/2024/day/13)

### Benchmark

#### Apple M3 Pro, 18 GB
`Completed in: 1.864ms`
```
Benchmark 1: cargo run --release --bin day_13
  Time (mean ± σ):     171.0 ms ±   1.6 ms    [User: 33.7 ms, System: 11.7 ms]
  Range (min … max):   168.5 ms … 174.2 ms    17 runs
```

#### Apple M2 Pro, 16 GB
``
```
```

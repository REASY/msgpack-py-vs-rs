msgpack-reader-rs
------
Simple application for reading [msgpack](https://msgpack.org/index.html) data in a streaming fashion.

# Dependencies

The project relies on several tools:

- [cargo](https://github.com/rust-lang/cargo) as the build system
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) to measure code coverage
- [criterion.rs](https://github.com/bheisler/criterion.rs) for benchmarking

# Development

The project requires the following tools configured on your developer machine:

- Cargo and Rust compiler installed, use [rustup](https://www.rust-lang.org/tools/install)
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov?tab=readme-ov-file#installation)

## Setup

### Install cargo-llvm-cov

```bash
cargo +stable install cargo-llvm-cov --locked
```

## Build the project

```bash
cargo build
```

## Run unit tests

```bash
cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.11s
     Running unittests src\main.rs (target\debug\deps\msgpack_reader_rs-a08cf319ced11bab.exe)

running 3 tests
test models::tests::test_from_list ... ok
test msgpack_parser::tests::test_item_msgpack_parser_should_work ... ok
test msgpack_parser::tests::test_msgpack_parser_should_work ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Get code coverage

```bash
cargo llvm-cov
info: cargo-llvm-cov currently setting cfg(coverage); you can opt-out it by passing --no-cfg-coverage
   Compiling msgpack-reader-rs v0.1.0 (C:\repos\github\REASY\msgpack-py-vs-rs\rust\msgpack-reader-rs)
    Finished test [unoptimized + debuginfo] target(s) in 1.74s
     Running unittests src\main.rs (target\llvm-cov-target\debug\deps\msgpack_reader_rs-a08cf319ced11bab.exe)

running 3 tests
test models::tests::test_from_list ... ok
test msgpack_parser::tests::test_item_msgpack_parser_should_work ... ok
test msgpack_parser::tests::test_msgpack_parser_should_work ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Filename                      Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
errors.rs                           4                 4     0.00%           3                 3     0.00%           5                 5     0.00%           0                 0         -
main.rs                            10                10     0.00%           2                 2     0.00%          22                22     0.00%           0                 0         -
models.rs                          10                 0   100.00%           2                 0   100.00%          35                 0   100.00%           0                 0         -
msgpack_parser.rs                  55                12    78.18%          11                 1    90.91%         121                 8    93.39%           0                 0         -
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                              79                26    67.09%          18                 6    66.67%         183                35    80.87%           0                 0         -
PS C:\repos\github\REASY\msgpack-py-vs-rs\rust\msgpack-reader-rs>
```

## Run code formatting

```bash
cargo fmt
```

## Run lint warnings fixer

```bash
cargo fix
```

## Run benchmark

```bash
cargo bench --bench in_memory_stream_benchmark --  --verbose
```

# Benchmark results

[msgpack-core/benches/in_memory_stream_benchmark.rs](msgpack-core/benches/in_memory_stream_benchmark.rs) contains a
benchmark that
reads generated msgpack file with **10000** messages
from [msgpack-core/test_resources/10000_items.msgpack](msgpack-core/test_resources/10000_items.msgpack) **into memory**
and uses it to
benchmark [ItemMsgPackParser](msgpack-core/src/msgpack_parser.rs). I'm using memory instead of file on file system to
reduce the IO effects on the benchmark. Average throughput (messages per second) in the table below is derived from
average time from
the benchmark: `1000 (ms in sec) / (average time in ms / 10000 messages)` which
is `1000 (ms in sec) * 10000 (messages) / average time (ms)`

## Environment

- Host OS: Microsoft Windows 10 Pro N x64 [Version 22H2, OS build 19045.4355]
- CPU: AMD Ryzen 9 3950X 16-Core Processor 3.5 GHz
- Memory: DDR4-2933 GHz 32 GB

| OS                          | Memory allocator                                             | Median, ms | MAD, µs | Average, ms | SD, µs | Average throughput msg/s | Average throughput, MBytes/s | Faster than baseline (avg), times |
|-----------------------------|--------------------------------------------------------------|------------|---------|-------------|--------|--------------------------|------------------------------|-----------------------------------|
| Windows 10 Pro N (Baseline) | Default                                                      | 4.2094     | 46.640  | 4.2093      | 46.862 | 2375691.92               | 126.74                       | **1**                             |
| Windows 10 Pro N            | [snmalloc-rs](https://github.com/SchrodingerZhu/snmalloc-rs) | 2.1603     | 19.505  | 2.1602      | 15.893 | 4628986.71               | 246.96                       | **1.948**                         |
| Ubuntu 24.04 LTS on WSL2    | Default                                                      | 2.7313     | 18.773  | 2.7310      | 18.491 | 3661662.39               | 195.35                       | **1.541**                         |
| Ubuntu 24.04 LTS on WSL2    | [snmalloc-rs](https://github.com/SchrodingerZhu/snmalloc-rs) | 2.1762     | 17.414  | 2.1765      | 15.965 | 4594532.50               | 245.12                       | **1.933**                         |
| Ubuntu 24.04 LTS on WSL2    | [jemallocator](https://github.com/tikv/jemallocator)         | 2.4596     | 17.084  | 2.4609      | 15.321 | 4063553.98               | 216.79                       | **1.710**                         |

Note:

- Used statistics from upper bound
- MAD is Median absolute deviation
- SD is Standard deviation
- Average throughput in MBytes/s calculated from average message size = 559439 bytes (total size of 10000 messages) /
  10000 messages = 55.9439 bytes/msg

## Windows 10 Pro N

### Default allocator

```bash
cargo bench --bench in_memory_stream_benchmark --  --verbose
   Compiling msgpack-core v0.1.1 (C:\repos\github\REASY\msgpack-py-vs-rs\rust\msgpack-core)
    Finished `bench` profile [optimized] target(s) in 1.94s
     Running benches\in_memory_stream_benchmark.rs (target\release\deps\in_memory_stream_benchmark-58b15471cc874faf.exe)
Gnuplot not found, using plotters backend
Benchmarking in_memory_stream_benchmark for 10000 messages
Benchmarking in_memory_stream_benchmark for 10000 messages: Warming up for 3.0000 s
Benchmarking in_memory_stream_benchmark for 10000 messages: Collecting 100 samples in estimated 5.0155 s (1200 iterations)
Benchmarking in_memory_stream_benchmark for 10000 messages: Analyzing
in_memory_stream_benchmark for 10000 messages
                        time:   [4.1932 ms 4.2012 ms 4.2093 ms]
                        change: [+94.303% +94.733% +95.189%] (p = 0.00 < 0.05)
                        Performance has regressed.
mean   [4.1932 ms 4.2093 ms] std. dev.      [33.655 µs 46.862 µs]
median [4.1907 ms 4.2094 ms] med. abs. dev. [29.769 µs 46.640 µs]
```

### [snmalloc-rs](https://github.com/SchrodingerZhu/snmalloc-rs) allocator

```bash
cargo bench --features allocator-snmalloc --bench in_memory_stream_benchmark --  --verbose
   Compiling msgpack-core v0.1.1 (C:\repos\github\REASY\msgpack-py-vs-rs\rust\msgpack-core)
    Finished `bench` profile [optimized] target(s) in 1.93s
     Running benches\in_memory_stream_benchmark.rs (target\release\deps\in_memory_stream_benchmark-3ae9f70d69127a08.exe)
Gnuplot not found, using plotters backend
Benchmarking in_memory_stream_benchmark for 10000 messages
Benchmarking in_memory_stream_benchmark for 10000 messages: Warming up for 3.0000 s
Benchmarking in_memory_stream_benchmark for 10000 messages: Collecting 100 samples in estimated 5.1661 s (2400 iterations)
Benchmarking in_memory_stream_benchmark for 10000 messages: Analyzing
in_memory_stream_benchmark for 10000 messages
                        time:   [2.1547 ms 2.1574 ms 2.1602 ms]
                        change: [-48.250% -48.105% -47.959%] (p = 0.00 < 0.05)
                        Performance has improved.
mean   [2.1547 ms 2.1602 ms] std. dev.      [12.585 µs 15.893 µs]
median [2.1541 ms 2.1603 ms] med. abs. dev. [11.540 µs 19.505 µs]
```

#### Rust compiler version

```bash
rustc --version --verbose
rustc 1.78.0 (9b00956e5 2024-04-29)
binary: rustc
commit-hash: 9b00956e56009bab2aa15d7bff10916599e3d6d6
commit-date: 2024-04-29
host: x86_64-pc-windows-msvc
release: 1.78.0
LLVM version: 18.1.2
```

## Ubuntu 24.04 LTS on WSL2

### Default allocator

```bash
cargo bench --bench in_memory_stream_benchmark --  --verbose
    Finished `bench` profile [optimized] target(s) in 1m 14s
     Running benches/in_memory_stream_benchmark.rs (target/release/deps/in_memory_stream_benchmark-ef326d3d9792adad)
Gnuplot not found, using plotters backend
Benchmarking in_memory_stream_benchmark for 10000 messages
Benchmarking in_memory_stream_benchmark for 10000 messages: Warming up for 3.0000 s
Benchmarking in_memory_stream_benchmark for 10000 messages: Collecting 100 samples in estimated 5.1786 s (1900 iterations)
Benchmarking in_memory_stream_benchmark for 10000 messages: Analyzing
in_memory_stream_benchmark for 10000 messages
                        time:   [2.7249 ms 2.7280 ms 2.7310 ms]
                        change: [+0.1953% +0.3755% +0.5540%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
mean   [2.7249 ms 2.7310 ms] std. dev.      [13.133 µs 18.491 µs]
median [2.7233 ms 2.7313 ms] med. abs. dev. [12.621 µs 18.773 µs]
```

### [snmalloc-rs](https://github.com/SchrodingerZhu/snmalloc-rs) allocator

```bash
cargo bench --features allocator-snmalloc --bench in_memory_stream_benchmark --  --verbose
   Compiling msgpack-core v0.1.1 (/mnt/c/repos/github/REASY/msgpack-py-vs-rs/rust/msgpack-core)
    Finished `bench` profile [optimized] target(s) in 8.00s
     Running benches/in_memory_stream_benchmark.rs (target/release/deps/in_memory_stream_benchmark-db30736c8c510f04)
Gnuplot not found, using plotters backend
Benchmarking in_memory_stream_benchmark for 10000 messages
Benchmarking in_memory_stream_benchmark for 10000 messages: Warming up for 3.0000 s
Benchmarking in_memory_stream_benchmark for 10000 messages: Collecting 100 samples in estimated 5.0503 s (2300 iterations)
Benchmarking in_memory_stream_benchmark for 10000 messages: Analyzing
in_memory_stream_benchmark for 10000 messages
                        time:   [2.1710 ms 2.1737 ms 2.1765 ms]
                        change: [-11.724% -11.572% -11.437%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
mean   [2.1710 ms 2.1765 ms] std. dev.      [11.837 µs 15.965 µs]
median [2.1700 ms 2.1762 ms] med. abs. dev. [11.501 µs 17.414 µs]
```

### [jemallocator](https://github.com/tikv/jemallocator) allocator

```bash
cargo bench --features allocator-jemalloc --bench in_memory_stream_benchmark --  --verbose
   Compiling tikv-jemalloc-sys v0.5.4+5.3.0-patched
   Compiling snmalloc-sys v0.3.5
   Compiling snmalloc-rs v0.3.5
   Compiling tikv-jemallocator v0.5.4
   Compiling msgpack-core v0.1.1 (/mnt/c/repos/github/REASY/msgpack-py-vs-rs/rust/msgpack-core)
    Finished `bench` profile [optimized] target(s) in 1m 08s
     Running benches/in_memory_stream_benchmark.rs (target/release/deps/in_memory_stream_benchmark-c2da915719955f3f)
Gnuplot not found, using plotters backend
Benchmarking in_memory_stream_benchmark for 10000 messages
Benchmarking in_memory_stream_benchmark for 10000 messages: Warming up for 3.0000 s
Benchmarking in_memory_stream_benchmark for 10000 messages: Collecting 100 samples in estimated 5.2037 s (2100 iterations)
Benchmarking in_memory_stream_benchmark for 10000 messages: Analyzing
in_memory_stream_benchmark for 10000 messages
                        time:   [2.4556 ms 2.4582 ms 2.4609 ms]
                        change: [-10.020% -9.8884% -9.7459%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
mean   [2.4556 ms 2.4609 ms] std. dev.      [11.677 µs 15.321 µs]
median [2.4538 ms 2.4596 ms] med. abs. dev. [9.8766 µs 17.084 µs]
```

#### Rust compiler version

```bash
rustc --version --verbose
rustc 1.78.0 (9b00956e5 2024-04-29)
binary: rustc
commit-hash: 9b00956e56009bab2aa15d7bff10916599e3d6d6
commit-date: 2024-04-29
host: x86_64-unknown-linux-gnu
release: 1.78.0
LLVM version: 18.1.2
```
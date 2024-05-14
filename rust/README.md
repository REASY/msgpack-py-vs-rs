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

| OS                       | Median, ms | MAD, µs | Average, ms | SD, µs | Average throughput msg/s |
|--------------------------|------------|---------|-------------|--------|--------------------------|
| Windows 10 Pro N         | 4.3529     | 54.492  | 4.3563      | 49.052 | 2295526.0                |
| Ubuntu 24.04 LTS on WSL2 | 2.8884     | 26.320  | 2.8924      | 43.566 | 3457336.46               |

Note:

- Used statistics from upper bound
- MAD is Median absolute deviation
- SD is Standard deviation

## Windows 10 Pro N

```bash
cargo bench --bench in_memory_stream_benchmark --  --verbose
    Finished bench [optimized] target(s) in 0.14s
     Running benches\in_memory_stream_benchmark.rs (target\release\deps\in_memory_stream_benchmark-b94fe03392101411.exe)
Gnuplot not found, using plotters backend
Benchmarking in_memory_stream_benchmark for 10000 messages
Benchmarking in_memory_stream_benchmark for 10000 messages: Warming up for 3.0000 s
Benchmarking in_memory_stream_benchmark for 10000 messages: Collecting 100 samples in estimated 5.2749 s (1200 iterations)
Benchmarking in_memory_stream_benchmark for 10000 messages: Analyzing
in_memory_stream_benchmark for 10000 messages
                        time:   [4.3391 ms 4.3476 ms 4.3563 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
mean   [4.3391 ms 4.3563 ms] std. dev.      [37.719 µs 49.052 µs]
median [4.3290 ms 4.3529 ms] med. abs. dev. [33.531 µs 54.492 µs]
```

#### Rust compiler version

```bash
rustc --version --verbose
rustc 1.77.2 (25ef9e3d8 2024-04-09)
binary: rustc
commit-hash: 25ef9e3d85d934b27d9dada2f9dd52b1dc63bb04
commit-date: 2024-04-09
host: x86_64-pc-windows-msvc
release: 1.77.2
LLVM version: 17.0.6
```

## Ubuntu 24.04 LTS on WSL2

```bash
 cargo bench --bench in_memory_stream_benchmark --  --verbose
    Finished bench [optimized] target(s) in 1.16s
     Running benches/in_memory_stream_benchmark.rs (target/release/deps/in_memory_stream_benchmark-d01bab4a9f92a6b6)
Gnuplot not found, using plotters backend
Benchmarking in_memory_stream_benchmark for 10000 messages
Benchmarking in_memory_stream_benchmark for 10000 messages: Warming up for 3.0000 s
Benchmarking in_memory_stream_benchmark for 10000 messages: Collecting 100 samples in estimated 5.2367 s (1800 iterations)
Benchmarking in_memory_stream_benchmark for 10000 messages: Analyzing
in_memory_stream_benchmark for 10000 messages
                        time:   [2.8810 ms 2.8860 ms 2.8924 ms]
                        change: [-33.809% -33.619% -33.433%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
mean   [2.8810 ms 2.8924 ms] std. dev.      [16.600 µs 43.566 µs]
median [2.8765 ms 2.8884 ms] med. abs. dev. [15.945 µs 26.320 µs]
```

#### Rust compiler version

```bash
rustc --version --verbose
rustc 1.77.2 (25ef9e3d8 2024-04-09)
binary: rustc
commit-hash: 25ef9e3d85d934b27d9dada2f9dd52b1dc63bb04
commit-date: 2024-04-09
host: x86_64-unknown-linux-gnu
release: 1.77.2
LLVM version: 17.0.6
```
Compare Rust and Python when parsing msgpack
---------

The purpose of this repository is to show the difference in runtime and memory consumption for an app that parse data
from MsgPack files.

* Python version is located [here](python/README.md)
* Rust version is located [here](rust/README.md)

According to the benchmarks, Rust implementation is **35.05** times faster than Python

| OS                       | Language                                                                        | Median, ms | MAD, µs | Average, ms | SD, µs | Average throughput msg/s |
|--------------------------|---------------------------------------------------------------------------------|------------|---------|-------------|--------|--------------------------|
| Ubuntu 24.04 LTS on WSL2 | Rust 1.78.0 (with [snmalloc-rs](https://github.com/SchrodingerZhu/snmalloc-rs)) | 2.1762     | 17.414  | 2.1765      | 15.965 | 4594532.50               |
| Ubuntu 24.04 LTS on WSL2 | CPython, 3.12.3 (64-bit), compiled with GCC 13.2.0                              | 76.2       | 400     | 76.3        | 600    | 131233.59                |

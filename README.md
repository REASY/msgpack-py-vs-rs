Compare Rust and Python when parsing msgpack
---------

The purpose of this repository is to show the difference in runtime and memory consumption for an app that parse data
from MsgPack files.

* Python version is located [here](python/README.md)
* Rust version is located [here](rust/README.md)

According to the benchmarks, Rust implementation is **26.34** times faster than Python

| OS                       | Language                                           | Median, ms | MAD, µs | Average, ms | SD, µs | Average throughput msg/s |
|--------------------------|----------------------------------------------------|------------|---------|-------------|--------|--------------------------|
| Ubuntu 24.04 LTS on WSL2 | Rust 1.77.2                                        | 2.8884     | 26.320  | 2.8924      | 43.566 | 3457336.46               |
| Ubuntu 24.04 LTS on WSL2 | CPython, 3.12.3 (64-bit), compiled with GCC 13.2.0 | 76.2       | 0.4     | 76.3        | 0.6    | 131233.59                |

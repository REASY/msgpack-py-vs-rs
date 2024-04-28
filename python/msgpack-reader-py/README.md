msgpack-reader-py
------
Simple application for reading [msgpack](https://msgpack.org/index.html) data in a streaming fashion. This Python
utility is built for efficiency and ease of use, offering a straightforward interface for working with msgpack data.

# Dependencies

The project relies on several tools and libraries:

- [Poetry](https://python-poetry.org) as the build system
- [pytest](https://docs.pytest.org/en/latest/) for unit testing
- [Coverage.py](https://coverage.readthedocs.io/) to measure code coverage
- [mypy](https://mypy-lang.org/) for static type checking
- [ruff](https://docs.astral.sh/ruff/) for code formatting

# Development

The project requires the following tools configured on your developer machine:

- [Python 3.12](https://www.python.org/downloads/release/python-3123/)
- [Poetry >= 1.8.2](https://python-poetry.org/docs/#installation)

## Setup

In the root folder of msgpack-reader-py run the following command to get all the dependencies installed

```bash
poetry install --with dev
```

## Run unit tests

```bash
poetry run pytest -v
================================================================== test session starts ==================================================================
platform linux -- Python 3.12.3, pytest-8.2.0, pluggy-1.5.0
rootdir: /mnt/c/repos/github/REASY/msgpack-py-vs-rs/python/msgpack-reader-py
configfile: pyproject.toml
testpaths: tests, integration
collected 4 items

tests/test_generator.py .                                                                                                                         [ 25%]
tests/test_model.py ..                                                                                                                            [ 75%]
tests/test_streaming_reader.py .                                                                                                                  [100%]

=================================================================== 4 passed in 0.13s ===================================================================
```

## Get code coverage

```bash
poetry run coverage run -m pytest && poetry run coverage report
....                                                                                                                                              [100%]
4 passed in 0.20s
Name                                    Stmts   Miss  Cover
-----------------------------------------------------------
msgpack_reader_py/__init__.py               0      0   100%
msgpack_reader_py/generator.py             33      6    82%
msgpack_reader_py/model.py                 18      0   100%
msgpack_reader_py/streaming_reader.py      33      4    88%
tests/__init__.py                           0      0   100%
tests/test_generator.py                    19      0   100%
tests/test_model.py                        15      0   100%
tests/test_streaming_reader.py             13      0   100%
-----------------------------------------------------------
TOTAL                                     131     10    92%
```

## Run code formatting

```bash
poetry run ruff format
```

## Run type checking

```bash
poetry run mypy msgpack_reader_py
```

## Run benchmark

```bash
poetry run python msgpack_reader_py/streaming_reader_bench.py --stats -o benchmark.result.json
```

# Benchmarking results

[msgpack_reader_py/streaming_reader_bench.py](msgpack_reader_py/streaming_reader_bench.py) contains a benchmark that
generates msgpack file with **10000** messages, reads it back **into memory** and uses it to
benchmark [StreamingReader](msgpack_reader_py/streaming_reader.py). I'm using memory instead of file on file system to
reduce the IO effects on the benchmark. Average throughput (messages per second) in the table below is derived from average time from
the benchmark: `1000 (ms in sec) / (average time in ms / 10000 messages)` which is `1000 (ms in sec) * 10000 (messages) / average time (ms)`

## Environment

- Host OS: Microsoft Windows 10 Pro N x64 [Version 22H2, OS build 19045.4355]
- CPU: AMD Ryzen 9 3950X 16-Core Processor 3.5 GHz
- Memory: DDR4-2933 GHz 32 GB

| OS                       | Python                                                                             | Median, ms | MAD, ms | Average, ms | SD, ms | Average throughput msg/s |
|--------------------------|------------------------------------------------------------------------------------|------------|---------|-------------|--------|--------------------------|
| Windows 10 Pro N         | CPython, 3.12.3 (64-bit) revision f6650f9, compiled with MSC v.1938 64 bit (AMD64) | 103        | 0       | 104         | 2      | 96153.84                 |
| Ubuntu 24.04 LTS on WSL2 | CPython, 3.12.3 (64-bit), compiled with GCC 13.2.0                                 | 76.2       | 0.4     | 76.3        | 0.6    | 131233.59                |

- MAD is Median absolute deviation
- SD is Standard deviation

## Windows 10 Pro N

```bash
> poetry run python msgpack_reader_py/streaming_reader_bench.py --stats -o windows_benchmark.result.json
.....................
Total duration: 10.9 sec
Start date: 2024-04-28 19:29:37
End date: 2024-04-28 19:29:55
Raw value minimum: 102 ms
Raw value maximum: 110 ms

Number of calibration run: 1
Number of run with values: 20
Total number of run: 21

Number of warmup per run: 1
Number of value per run: 3
Loop iterations per value: 1
Total number of values: 60

Minimum:         102 ms
Median +- MAD:   103 ms +- 0 ms
Mean +- std dev: 104 ms +- 2 ms
Maximum:         110 ms

  0th percentile: 102 ms (-2% of the mean) -- minimum
  5th percentile: 102 ms (-1% of the mean)
 25th percentile: 103 ms (-1% of the mean) -- Q1
 50th percentile: 103 ms (-0% of the mean) -- median
 75th percentile: 103 ms (-0% of the mean) -- Q3
 95th percentile: 108 ms (+4% of the mean)
100th percentile: 110 ms (+6% of the mean) -- maximum

Number of outlier (out of 102 ms..104 ms): 6

in_memory_stream_benchmark: Mean +- std dev: 104 ms +- 2 ms
```

## Ubuntu 24.04 LTS on WSL2

```bash
$ poetry run python msgpack_reader_py/streaming_reader_bench.py --stats -o doc/wsl2_benchmark.result.json
.....................
Total duration: 13.1 sec
Start date: 2024-04-28 20:19:19
End date: 2024-04-28 20:19:37
Raw value minimum: 151 ms
Raw value maximum: 155 ms

Number of calibration run: 1
Number of run with values: 20
Total number of run: 21

Number of warmup per run: 1
Number of value per run: 3
Loop iterations per value: 2
Total number of values: 60

Minimum:         75.6 ms
Median +- MAD:   76.2 ms +- 0.4 ms
Mean +- std dev: 76.3 ms +- 0.6 ms
Maximum:         77.6 ms

  0th percentile: 75.6 ms (-1% of the mean) -- minimum
  5th percentile: 75.6 ms (-1% of the mean)
 25th percentile: 75.8 ms (-1% of the mean) -- Q1
 50th percentile: 76.2 ms (-0% of the mean) -- median
 75th percentile: 76.7 ms (+0% of the mean) -- Q3
 95th percentile: 77.3 ms (+1% of the mean)
100th percentile: 77.6 ms (+2% of the mean) -- maximum

Number of outlier (out of 74.6 ms..77.9 ms): 0

in_memory_stream_benchmark: Mean +- std dev: 76.3 ms +- 0.6 ms
```
#!/usr/bin/env python3
import argparse
import io
import tempfile

import pyperf

from msgpack_reader_py.generator import generate
from msgpack_reader_py.model import Item
from msgpack_reader_py.streaming_reader import StreamingReader


def in_memory_stream_benchmark(msgpack_binary: bytes) -> int:
    binary_io = io.BytesIO(msgpack_binary)
    msgs = 0
    with StreamingReader(binary_io, mapper=Item.from_list) as rdr:
        for _ in rdr:
            msgs += 1
    return msgs


messages = 10000
seed = 42
with tempfile.NamedTemporaryFile(delete_on_close=False) as fp:
    args = argparse.Namespace(path=fp.name, messages=messages, seed=seed)
    generate(args.path, args.messages, args.seed)
    fp.close()

    with open(fp.name, "rb") as fh:
        read_msgpack_bytes = fh.read()
    runner = pyperf.Runner()
    runner.bench_func(
        "in_memory_stream_benchmark", in_memory_stream_benchmark, read_msgpack_bytes
    )

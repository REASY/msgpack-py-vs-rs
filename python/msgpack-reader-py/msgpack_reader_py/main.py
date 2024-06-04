import time
from typing import Any
from msgpack_reader_py.streaming_reader import StreamingReader


def main() -> None:
    msgs = 0
    t0 = time.time()

    def identity(x: Any) -> Any:
        return x

    with StreamingReader(
        "C:\\repos\\github\\REASY\\msgpack-py-vs-rs\\python\\msgpack-reader-py\\msgpack_reader_py\\1.msgpack",
        mapper=identity,
    ) as rdr:
        for _ in rdr:
            msgs += 1
    dt_sec = time.time() - t0
    print(f"Read {msgs} messages in {dt_sec:.2f} seconds")


if __name__ == "__main__":
    main()

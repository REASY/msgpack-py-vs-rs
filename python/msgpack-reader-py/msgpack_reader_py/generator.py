import argparse
import random
import string
from typing import Iterator, BinaryIO

import msgpack

from msgpack_reader_py.model import Item, item_to_list


def get_iter(messages: int, seed: int) -> Iterator[Item]:
    rnd = random.Random(x=seed)
    for i in range(messages):
        process_id = rnd.randint(0, 65535)
        thread_id = rnd.randint(0, 65535)
        line = rnd.randint(0, 65535)
        timestamp_ns = rnd.randint(1714290559598901400, 1777333982598901400)
        value: float = rnd.uniform(0.0, 1234.0)
        filename = "".join(rnd.choices(string.ascii_uppercase + string.digits, k=8))
        path = "".join(rnd.choices(string.ascii_uppercase + string.digits, k=15))
        item = Item(
            id=i,
            process_id=process_id,
            thread_id=thread_id,
            timestamp_ns=timestamp_ns,
            line=line,
            value=value,
            filename=filename,
            path=path,
        )
        yield item


def generate(path: str, messages: int, seed: int) -> None:
    iterator = get_iter(messages, seed)
    with open(path, "wb") as f:
        write_items(f, iterator)


def write_items(io: BinaryIO, iterator: Iterator[Item]) -> None:
    packer = msgpack.Packer(
        default=item_to_list,
    )
    for item in iterator:
        io.write(packer.pack(item))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--path", type=str, help="Path to file", required=True)
    parser.add_argument(
        "--messages", type=int, help="Number of messages", required=True
    )
    parser.add_argument("--seed", type=int, help="Random seed", default=42)
    args = parser.parse_args()
    generate(args.path, args.messages, args.seed)

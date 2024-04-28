import io

from msgpack_reader_py.generator import get_iter, write_items
from msgpack_reader_py.model import Item
from msgpack_reader_py.streaming_reader import StreamingReader


def test_streaming_reader() -> None:
    expected_arr = list(get_iter(5, 42))

    f = io.BytesIO()
    write_items(f, iter(expected_arr))
    # Seek to the beginning
    f.seek(0)

    # Create StreamingReader to read all the messages
    with StreamingReader(f, mapper=Item.from_list) as rdr:
        msgs = list(rdr)

    assert len(expected_arr) == len(msgs)
    assert expected_arr == msgs

import argparse
import tempfile

import msgpack

from msgpack_reader_py.generator import generate


def test_generate():
    messages = 3
    seed = 42
    with tempfile.NamedTemporaryFile(delete_on_close=False) as fp:
        generate(path=fp.name, messages=messages, seed=seed)

        unpacker = msgpack.Unpacker(fp, raw=False)
        id = 0
        expected_arr = [
            [
                0,
                14592,
                3278,
                1730374586108998696,
                36048,
                172.1898038000276,
                "D0TVBDIV",
                "UZZPQK51FPKH1DN",
            ],
            [
                1,
                47052,
                45082,
                1717421495440027159,
                34671,
                900.4890247801712,
                "T9NT3W5U",
                "ZBIKCIDKWNNHJ7X",
            ],
            [
                2,
                22431,
                32087,
                1741633203858925578,
                21417,
                333.11561825179905,
                "7YHL1C32",
                "OC6UZHR5XFF0T0P",
            ],
        ]
        for v in unpacker:
            assert type(v) is list
            assert len(v) == 8
            assert v[0] == id
            print(v)
            assert v == expected_arr[id]
            id += 1

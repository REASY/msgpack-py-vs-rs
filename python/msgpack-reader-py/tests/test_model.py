from msgpack_reader_py.model import Item, item_to_list

one_item = Item(
    id=1,
    process_id=2,
    thread_id=3,
    timestamp_ns=4,
    line=5,
    value=6.0,
    filename="test.txt",
    path="/tmp",
)


def test_item_to_list() -> None:
    arr = item_to_list(one_item)
    assert len(arr) == 8
    assert arr[0] == one_item.id
    assert arr[1] == one_item.process_id
    assert arr[2] == one_item.thread_id
    assert arr[3] == one_item.timestamp_ns
    assert arr[4] == one_item.line
    assert arr[5] == one_item.value
    assert arr[6] == one_item.filename
    assert arr[7] == one_item.path


def test_from_list() -> None:
    assert Item.from_list(item_to_list(one_item)) == one_item

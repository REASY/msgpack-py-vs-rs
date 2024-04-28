import typing
from dataclasses import dataclass


@dataclass
@typing.final
class Item:
    id: int
    process_id: int
    thread_id: int
    timestamp_ns: int
    line: int
    value: float
    filename: str
    path: str

    @classmethod
    def from_list(cls, items: list[typing.Any]) -> typing.Self:
        return Item(
            items[0],
            items[1],
            items[2],
            items[3],
            items[4],
            items[5],
            items[6],
            items[7],
        )


def item_to_list(item: Item) -> list:
    return [
        item.id,
        item.process_id,
        item.thread_id,
        item.timestamp_ns,
        item.line,
        item.value,
        item.filename,
        item.path,
    ]

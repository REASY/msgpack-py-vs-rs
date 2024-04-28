import io
import logging
import typing
from typing import Callable, TypeVar

from msgpack import Unpacker

logger = logging.getLogger(__name__)

T = TypeVar("T")


class StreamingReader(typing.Generic[T]):
    file_path_or_binary: typing.Union[str, typing.BinaryIO]

    def __init__(
        self,
        file_path_or_binary: typing.Union[str, typing.BinaryIO],
        mapper: Callable[[typing.Any], T],
    ) -> None:
        self.file_path_or_binary = file_path_or_binary
        self.mapper = mapper

    def __enter__(self) -> typing.Self:
        # Open provided `self.file_path_or_binary` as a file if it is string, meaning path to a file
        if isinstance(self.file_path_or_binary, str):
            self.file: typing.Optional[typing.BinaryIO] = open(
                self.file_path_or_binary, "rb"
            )
        elif isinstance(self.file_path_or_binary, io.BytesIO):
            # if it is io.BytesIO, use it as file as well, it follows file's API
            self.file = self.file_path_or_binary
        else:
            raise ValueError("file_path_or_binary must be str or io.BytesIO")
        self.unpacker = Unpacker(self.file, raw=False)
        return self

    def __exit__(self, exc_type, exc_val, exc_tb) -> None:
        try:
            if self.file:
                self.file.close()
        except Exception as ex:
            logger.exception(f"Error when closing {self.file_path_or_binary}", ex)
        finally:
            self.file = None

    def __iter__(self) -> typing.Self:
        return self

    def __next__(self) -> T:
        # Read next
        unpacked = self.unpacker.next()
        # Apply mapper to convert it to the type T
        mapped = self.mapper(unpacked)
        return mapped

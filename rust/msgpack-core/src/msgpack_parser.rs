use crate::errors;
use crate::errors::{Error, ErrorKind};
use crate::models::Item;
use rmpv::Value;
use std::io::ErrorKind::UnexpectedEof;
use tracing::warn;

pub struct MsgPackParser<R> {
    reader: R,
}

impl<R: std::io::Read> MsgPackParser<R> {
    pub fn new(reader: R) -> Self {
        MsgPackParser { reader }
    }

    pub fn parse(
        &mut self,
        mut on_next: impl FnMut(&Value) -> errors::Result<()>,
    ) -> errors::Result<()> {
        loop {
            match rmpv::decode::read_value(&mut self.reader) {
                Ok(value) => on_next(&value)?,
                Err(err) => {
                    if err.kind() != UnexpectedEof {
                        warn!("Failed with err: {err}, kind: {}", err.kind());
                        return Err(errors::Error::from(err));
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}

pub struct ItemMsgPackParser<R> {
    parser: MsgPackParser<R>,
}

impl<R: std::io::Read> ItemMsgPackParser<R> {
    pub fn new(reader: R) -> Self {
        ItemMsgPackParser {
            parser: MsgPackParser::new(reader),
        }
    }

    pub fn parse(
        &mut self,
        mut on_next: impl FnMut(&Item) -> errors::Result<()>,
    ) -> errors::Result<()> {
        self.parser.parse(|value| match value {
            Value::Array(arr) => {
                let item = Item::from_list(arr.as_slice());
                on_next(&item)?;
                Ok(())
            }
            x => {
                return Err(Error::from(ErrorKind::ItemMsgPackParser(format!(
                    "Expected Array type but got {}",
                    x
                ))));
            }
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Alphanumeric;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;
    use rmpv::encode;
    use std::io::{Cursor, Seek, SeekFrom};

    fn generate(n: usize, seed: u64) -> Vec<Value> {
        let mut rnd = ChaCha8Rng::seed_from_u64(seed);
        let mut xs: Vec<Value> = Vec::new();
        for id in 0..n {
            let process_id: i32 = rnd.gen();
            let thread_id: i32 = rnd.gen();
            let line: i32 = rnd.gen();
            let timestamp_ns: i64 = rnd.gen();
            let value: f32 = rnd.gen();
            let filename: String = (&mut rnd)
                .sample_iter(&Alphanumeric)
                .take(8)
                .map(char::from)
                .collect();
            let path: String = (&mut rnd)
                .sample_iter(&Alphanumeric)
                .take(15)
                .map(char::from)
                .collect();
            let arr: Vec<Value> = vec![
                Value::from(id),
                Value::from(process_id),
                Value::from(thread_id),
                Value::from(line),
                Value::from(timestamp_ns),
                Value::from(value),
                Value::from(filename),
                Value::from(path),
            ];
            xs.push(Value::Array(arr));
        }
        return xs;
    }

    #[test]
    fn test_msgpack_parser_should_work() -> errors::Result<()> {
        let data = generate(10, 42);

        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        for v in &data {
            encode::write_value(&mut cursor, &v).unwrap();
        }

        cursor.seek(SeekFrom::Start(0))?;

        let mut id: usize = 0;
        let mut parser = MsgPackParser::new(cursor);
        parser.parse(|value: &Value| {
            let expected = &data[id];
            assert_eq!(expected, value);

            id += 1;

            Ok(())
        })?;

        Ok(())
    }

    #[test]
    fn test_item_msgpack_parser_should_work() -> errors::Result<()> {
        let data = generate(10, 42);

        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        for v in &data {
            encode::write_value(&mut cursor, &v).unwrap();
        }

        cursor.seek(SeekFrom::Start(0))?;

        let mut id: usize = 0;
        let mut parser = ItemMsgPackParser::new(cursor);
        parser.parse(|value: &Item| {
            let expected = Item::from_list(&data[id].as_array().unwrap().as_slice());
            assert_eq!(&expected, value);

            id += 1;

            Ok(())
        })?;

        Ok(())
    }
}

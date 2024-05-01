use msgpack_core::errors::Result;
use msgpack_core::models::Item;
use msgpack_core::msgpack_parser::ItemMsgPackParser;
use shadow_rs::shadow;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

shadow!(build);

pub const APP_VERSION: &str = shadow_rs::formatcp!(
    "{} ({} {}), build_env: {}, {}, {}",
    build::PKG_VERSION,
    build::SHORT_COMMIT,
    build::BUILD_TIME,
    build::RUST_VERSION,
    build::RUST_CHANNEL,
    build::CARGO_VERSION
);

fn main() -> Result<()> {
    println!("APP_VERSION: {APP_VERSION}");
    let s = Instant::now();
    let f = File::open("C:/repos/github/REASY/msgpack-py-vs-rs/python/msgpack-reader-py/msgpack_reader_py/1.msgpack")?;
    let rdr = BufReader::new(f);

    let mut msgs: usize = 0;
    let mut count: i64 = 0;

    let mut parser = ItemMsgPackParser::new(rdr);
    parser.parse(&mut |value: &Item| {
        count += value.id as i64;
        msgs += 1;

        Ok(())
    })?;

    println!("Read {msgs} messages in {} ms", s.elapsed().as_millis());

    Ok(())
}

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
    let s = Instant::now();
    let f = File::open("C:\\Users\\User\\Downloads\\flax_model.msgpack")?;
    let rdr = BufReader::new(f);

    let mut msgs: usize = 0;
    let mut sum_of_ids: i64 = 0;

    let mut parser: ItemMsgPackParser<BufReader<File>> = ItemMsgPackParser::new(rdr);
    let on_next = |value: &Item| {
        sum_of_ids += value.id as i64;
        msgs += 1;

        Ok(())
    };
    parser.parse(on_next)?;
    println!(
        "Read {msgs} messages, sum_of_ids is {sum_of_ids} in {} ms",
        s.elapsed().as_millis()
    );

    Ok(())
}

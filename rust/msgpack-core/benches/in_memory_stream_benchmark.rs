use criterion::{black_box, criterion_group, criterion_main, Criterion};
use msgpack_core::msgpack_parser::ItemMsgPackParser;
use std::io::Cursor;

fn parse_items(bytes: &[u8]) {
    let mut parser = ItemMsgPackParser::new(Cursor::new(bytes));
    parser
        .parse(|v| {
            // Let's consume `v` using `black_box` to make sure compiler won't get rid of unused arg
            black_box(v);
            Ok(())
        })
        .unwrap();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // The file contains exact same data as for Python benchmark
    let bytes = include_bytes!("../test_resources/10000_items.msgpack").to_vec();

    c.bench_function("in_memory_stream_benchmark for 10000 messages", |b| {
        b.iter(|| parse_items(bytes.as_slice()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

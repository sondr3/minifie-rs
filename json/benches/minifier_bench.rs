use criterion::*;
use json::minify::Minify;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn criterion_benchmark(c: &mut Criterion) {
    let file = File::open("./tests/fixtures/large.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut string = String::new();
    buf_reader.read_to_string(&mut string).unwrap();

    c.bench_function("minify", move |b| {
        b.iter_with_setup(
            || string.as_str(),
            |contents| format!("{}", Minify::new(contents)),
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

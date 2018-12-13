use json::minify::Minify;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn read_file(filename: &str) -> String {
    let file = File::open(filename).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    contents
}

#[test]
fn large() {
    let large = read_file("./tests/fixtures/large.json");
    let large_correct = read_file("./tests/fixtures/large_correct.json");
    let minified = Minify::new(large.as_str());

    assert_eq!(large_correct.as_str(), format!("{}", minified));
}

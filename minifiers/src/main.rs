use std::path::PathBuf;
use structopt::StructOpt;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use json::minify::Minify;

#[derive(Debug, StructOpt)]
#[structopt(name = "minifiers")]
struct Opt {
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let opt = Opt::from_args();
    for file in &opt.files {
        if file.extension().unwrap() == "json" {
            let file = File::open(file)?;
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents)?;
            let minified = Minify::new(contents.as_str());
            println!("{}", minified);
        }
    }

    Ok(())
}

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "minifiers")]
struct Opt {
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    for file in &opt.files {
        if file.extension().unwrap() == "json" {
            println!("its JSON BABY");
        }
    }
}

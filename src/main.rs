use aoc_downloader::download_day;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name="aoc-downloader")]
struct Opt {
    /// Day that should be retrieved
    #[structopt(short, long)]
    day: u32,
    /// Path where input is stored as input<day>.txt
    #[structopt(short, long, default_value = "input")]
    path: String,
}

fn main() {
    let options = Opt::from_args();
    if !std::path::Path::new(&format!("{}/input{}.txt", options.path, options.day)).exists() {
        download_day(options.day, &options.path).unwrap(); 
    }
}

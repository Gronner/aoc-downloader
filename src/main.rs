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
    download_day(options.day, &options.path).unwrap(); 
}

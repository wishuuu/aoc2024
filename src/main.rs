use clap::Parser;
use days::Runner;

mod days;
mod structures;
mod utils;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long)]
    pub day: Option<u8>,
    #[clap(short, long)]
    pub test: bool,
}

fn main() {
    let args = Args::parse();
    let runner = Runner::new(args);

    runner.run();
}

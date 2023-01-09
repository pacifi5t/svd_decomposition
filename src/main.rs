use clap::{Parser, ValueHint};
use clap::builder::RangedU64ValueParser;

#[derive(Parser, Debug)]
struct Args {
    /// Input file
    #[arg(value_hint = ValueHint::FilePath)]
    ifile: String,

    /// Compression level (0 - none, 9 - max)
    #[arg(short = 'c', value_parser = RangedU64ValueParser::<u8>::new().range(0..=9))]
    comp: u8,

    /// Output file
    #[arg(short = 'o', value_hint = ValueHint::FilePath)]
    ofile: Option<String>
}

fn main() {
    let args = Args::parse();
    println!("Hello, world!");
}

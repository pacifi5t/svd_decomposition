use anyhow::Result;
use clap::builder::RangedU64ValueParser;
use clap::{arg, Parser, ValueHint};
use image::{DynamicImage, GenericImageView};
use nalgebra_sparse::{CooMatrix, CsrMatrix};

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
    ofile: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let colors = extract_colors(&image::open(args.ifile)?);
    Ok(())
}

fn extract_colors(image: &DynamicImage) -> Vec<CooMatrix<u8>> {
    let shape = (image.height() as usize, image.width() as usize);
    let mut matrices = vec![CooMatrix::<u8>::new(shape.0, shape.1); 4];

    for (col, row, color) in image.pixels() {
        for (i,each) in color.0.iter().enumerate() {
            matrices[i].push(row as usize, col as usize, *each);
        }
    }

    matrices
}

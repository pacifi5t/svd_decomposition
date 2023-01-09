use anyhow::Result;
use clap::builder::RangedU64ValueParser;
use clap::{arg, Parser, ValueHint};
use image::{DynamicImage, GenericImage, GenericImageView, ImageFormat, Rgba};
use nalgebra::DMatrix;
use nalgebra_sparse::na::SVD;

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

    let image = image::open(args.ifile)?;
    let colors = extract_color_matrices(&image);

    let max_rank = image.width().min(image.height());
    let nsingular = (max_rank as f32 * (10 - args.comp) as f32 / 10.0) as usize;

    let mut compressed_colors = vec![];
    for each in colors.iter().take(3) {
        let matrix = compress(each, nsingular);
        compressed_colors.push(matrix.iter().cloned().collect::<Vec<f64>>());
    }

    let mut pixels = vec![];
    for (i, each) in compressed_colors[0].iter().enumerate() {
        let each1 = compressed_colors[1][i];
        let each2 = compressed_colors[2][i];

        let r = (each.min(1.0) * u8::MAX as f64) as u8;
        let g = (each1.min(1.0) * u8::MAX as f64) as u8;
        let b = (each2.min(1.0) * u8::MAX as f64) as u8;
        pixels.push(Rgba([r, g, b, 255]))
    }

    let mut new_image = DynamicImage::new_rgb8(image.width(), image.height());
    for (i, each) in pixels.iter().enumerate() {
        let y = i as u32 / new_image.width();
        let x = i as u32 % new_image.width();
        new_image.put_pixel(x, y, *each);
    }

    let path = args.ofile.unwrap_or_else(|| "out.jpg".into());
    new_image.save_with_format(path, ImageFormat::Jpeg)?;
    Ok(())
}

fn extract_color_matrices(image: &DynamicImage) -> Vec<DMatrix<f64>> {
    let (nrows, ncols) = (image.height() as usize, image.width() as usize);
    let mut matrices = vec![vec![]; 3];

    for (_, _, color) in image.pixels() {
        for (i, each) in color.0.iter().take(3).enumerate() {
            matrices[i].push(*each as f64 / u8::MAX as f64);
        }
    }

    matrices
        .into_iter()
        .map(|e| DMatrix::from_vec(nrows, ncols, e))
        .collect()
}

fn compress(color_matrix: &DMatrix<f64>, nsingular: usize) -> DMatrix<f64> {
    let svd = SVD::new(color_matrix.to_owned(), true, true);
    let u = svd.u.unwrap();
    let vt = svd.v_t.unwrap();
    let mut a = DMatrix::<f64>::zeros(u.nrows(), vt.ncols());

    for i in 0..nsingular {
        let ui = u.index((.., i));
        let vi = vt.index((i, ..));
        let t = ui * vi;
        a += svd.singular_values[i] * t;
    }

    a
}

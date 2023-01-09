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
    let max_rank = image.width().min(image.height());
    let n_singular = (max_rank as f32 * (10 - args.comp) as f32 / 10.0) as usize;

    let mut comp_colors = vec![];
    for each in extract_color_matrices(&image).iter().take(3) {
        let matrix = compress(each, n_singular);
        comp_colors.push(matrix.iter().cloned().collect::<Vec<f64>>());
    }

    let path = args.ofile.unwrap_or_else(|| "out.jpg".into());
    create_new_image(image, &comp_colors).save_with_format(path, ImageFormat::Jpeg)?;
    Ok(())
}

fn create_new_image(image: DynamicImage, comp_colors: &[Vec<f64>]) -> DynamicImage {
    let mut pixels = vec![];
    for (i, each) in comp_colors[0].iter().enumerate() {
        let r = (each.min(1.0) * u8::MAX as f64) as u8;
        let g = (comp_colors[1][i].min(1.0) * u8::MAX as f64) as u8;
        let b = (comp_colors[2][i].min(1.0) * u8::MAX as f64) as u8;
        pixels.push(Rgba([r, g, b, 255]))
    }

    let mut new_image = DynamicImage::new_rgb8(image.width(), image.height());
    for (i, each) in pixels.iter().enumerate() {
        let y = i as u32 / new_image.width();
        let x = i as u32 % new_image.width();
        new_image.put_pixel(x, y, *each);
    }
    new_image
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

fn compress(color_matrix: &DMatrix<f64>, ns: usize) -> DMatrix<f64> {
    let svd = SVD::new(color_matrix.to_owned(), true, true);
    let (u, vt) = (svd.u.unwrap(), svd.v_t.unwrap());

    let mut a = DMatrix::<f64>::zeros(u.nrows(), vt.ncols());
    a.set_diagonal(&svd.singular_values);

    u.index((.., ..ns)) * a.index((..ns, ..ns)) * vt.index((..ns, ..))
}

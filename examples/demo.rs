use clap::{arg, Parser};
use nalgebra::{DMatrix, SVD};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

#[derive(Parser, Debug)]
struct Args {
    /// Number of rows
    rows: usize,

    /// Number of columns
    cols: usize,

    /// Number of singular values
    #[arg(short = 's', long = "singular")]
    sing: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let mat = random_matrix(args.rows, args.cols);

    let (u, vt, singular) = svd(mat.clone());
    let s_len = singular.nrows().min(singular.ncols());
    let ns = args.sing.unwrap_or(s_len).min(s_len);

    print!("Source matrix{}", mat);
    print!("U matrix{}", u.index((.., ..ns)));
    print!("Singular values{}", singular.index((..ns, ..ns)));
    print!("Vt matrix{}", vt.index((..ns, ..)));
}

fn random_matrix(nrows: usize, ncols: usize) -> DMatrix<f64> {
    let mut rng = Xoshiro256Plus::seed_from_u64(42);
    let vec = (0..nrows * ncols)
        .map(|_| round(rng.gen_range(0.0..1000.0)))
        .collect();
    DMatrix::from_vec(nrows, ncols, vec)
}

fn svd(mat: DMatrix<f64>) -> (DMatrix<f64>, DMatrix<f64>, DMatrix<f64>) {
    let svd = SVD::new(mat, true, true);
    let (mut u, mut vt) = (svd.u.unwrap(), svd.v_t.unwrap());
    let mut singular = DMatrix::zeros(u.nrows(), vt.ncols());
    singular.set_diagonal(&svd.singular_values);

    singular.iter_mut().for_each(|e| *e = round(*e));
    u.iter_mut().for_each(|e| *e = round(*e));
    vt.iter_mut().for_each(|e| *e = round(*e));
    (u, vt, singular)
}

fn round(num: f64) -> f64 {
    (num * 100.0).round() / 100.0
}

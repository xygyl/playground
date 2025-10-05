use rand::{distr::uniform::SampleUniform, Rng};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

pub fn gen_matrix<T>(rows: usize, cols: usize, min: T, max: T) -> Vec<Vec<T>>
where
    T: SampleUniform + Send + Sync + Copy + PartialOrd,
{
    assert!(rows > 0 && cols > 0, "rows and columns must be > 0");
    assert!(min <= max, "min must be <= max");
    let mut matrix = vec![vec![min; cols]; rows];
    matrix.par_iter_mut().for_each(|row| {
        row.iter_mut().for_each(|val| {
            *val = rand::rng().random_range(min..=max);
        });
    });
    matrix
}

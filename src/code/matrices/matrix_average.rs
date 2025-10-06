use crate::helper::{gen_matrix::gen_matrix, green_to_red_gradient::green_to_red_gradient};
use indicatif::ProgressBar;
use inquire::CustomType;
use rand::Rng;
use rayon::prelude::*;

const EDGE: usize = 10;
const MIN: u32 = 1000;
const MAX: u32 = 9999;

pub fn matrix_average() -> Option<()> {
    let iter: u32 = CustomType::new("Iterations:")
        .with_help_message("The number of iterations")
        .prompt()
        .ok()?;
    let pb = ProgressBar::new(iter as u64);

    let mut matrix = gen_matrix(EDGE, EDGE, MIN, MAX);

    print_matrix(&matrix);

    println!();
    if iter > 1 {
        for _ in 0..iter {
            matrix.par_iter_mut().for_each(|row| {
                row.iter_mut().for_each(|val| {
                    let rand = rand::rng().random_range(MIN..=MAX);
                    *val = (*val + rand) / 2;
                });
            });
            pb.inc(1);
        }
        pb.finish();
        println!();
        println!();
        print_matrix(&matrix);
    }
    Some(())
}

fn print_matrix(matrix: &Vec<Vec<u32>>) {
    let (min, max) = min_max(matrix);
    for row in matrix {
        for &val in row {
            let colored_val = green_to_red_gradient(val, MAX-MIN);
            print!("{} ", colored_val);
        }
        println!();
    }
    println!("min: {}", min);
    println!("max: {}", max);
    println!("diff: {}", max - min);
}

fn min_max(matrix: &Vec<Vec<u32>>) -> (u32, u32) {
    matrix
        .par_iter()
        .map(|row| {
            let min = *row.iter().min().unwrap();
            let max = *row.iter().max().unwrap();
            (min, max)
        })
        .reduce(
            || (u32::MAX, u32::MIN),
            |(min1, max1), (min2, max2)| (min1.min(min2), max1.max(max2)),
        )
}

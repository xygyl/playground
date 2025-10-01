use inquire::CustomType;
use rand::Rng;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

/// Creates an array of user-specified size and then transposes it, printing it before and after.
pub fn array_transpose() {
    let rows: usize = CustomType::new("Rows:")
        .with_help_message("Enter the desired number of rows")
        .prompt()
        .unwrap();

    let cols: usize = CustomType::new("Columns:")
        .with_help_message("Enter the desired number of columns")
        .prompt()
        .unwrap();
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; cols]; rows];

    matrix.par_iter_mut().for_each(|row| {
        row.iter_mut().for_each(|val| {
            *val = rand::rng().random_range(10..=99);
        });
    });

    println!("\nOriginal Matrix:");
    for row in &matrix {
        for &val in row {
            print!("{:>2} ", val);
        }
        println!();
    }

    let mut transpose: Vec<Vec<i32>> = vec![vec![0; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            transpose[j][i] = matrix[i][j];
        }
    }

    println!("\nTransposed Matrix:");
    for row in &transpose {
        for &val in row {
            print!("{:>2} ", val);
        }
        println!();
    }
}

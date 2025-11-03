use crate::helper::gen_matrix::gen_matrix;
use inquire::CustomType;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

fn print_matrix(matrix: &Vec<Vec<u32>>, message: &str) {
    println!("\n{}", message);
    for row in matrix {
        for val in row {
            print!("{:>2} ", val);
        }
        println!();
    }
}
/// Creates an matrix of user-specified size and then transposes it, printing it before and after.
pub fn matrix_transpose() -> Option<()> {
    let rows: usize = CustomType::new("Rows:")
        .with_help_message("Enter the desired number of rows")
        .prompt()
        .ok()?;

    let cols: usize = CustomType::new("Columns:")
        .with_help_message("Enter the desired number of columns")
        .prompt()
        .ok()?;
    let matrix = gen_matrix(rows, cols, 10, 99);

    println!("\nOriginal Matrix:");
    print_matrix(&matrix, "Original Matrix:");

    let mut transpose: Vec<Vec<u32>> = vec![vec![0; rows]; cols];
    transpose.par_iter_mut().enumerate().for_each(|(j, t_row)| {
        for i in 0..rows {
            t_row[i] = matrix[i][j];
        }
    });

    println!("\nTransposed Matrix:");
    print_matrix(&transpose, "Transposed Matrix:");
    Some(())
}

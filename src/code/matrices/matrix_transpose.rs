use crate::helper::gen_matrix::gen_matrix;
use inquire::CustomType;

/// Creates an matrix of user-specified size and then transposes it, printing it before and after.
pub fn matrix_transpose() {
    let rows: usize = CustomType::new("Rows:")
        .with_help_message("Enter the desired number of rows")
        .prompt()
        .unwrap();

    let cols: usize = CustomType::new("Columns:")
        .with_help_message("Enter the desired number of columns")
        .prompt()
        .unwrap();
    let matrix = gen_matrix(rows, cols, 10, 99);

    println!("\nOriginal Matrix:");
    for row in &matrix {
        for &val in row {
            print!("{:>2} ", val);
        }
        println!();
    }

    let mut transpose: Vec<Vec<u32>> = vec![vec![0; rows]; cols];

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

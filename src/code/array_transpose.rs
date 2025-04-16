use rand::Rng;
use text_io::read;

pub fn array_transpose() {
    // Get rows from user
    print!("\nEnter number of rows: ");
    let rows: usize = read!();

    // Get cols from user
    print!("Enter number of columns: ");
    let cols: usize = read!();

    // Allocate and fill matrix with random values
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; cols]; rows];
    for row in matrix.iter_mut() {
        for val in row.iter_mut() {
            *val = rand::rng().random_range(10..=99); // 10 to 99; ensures every number has two numbers
        }
    }

    // Print the original matrix
    println!("\nOriginal Matrix:");
    for row in &matrix {
        for &val in row {
            print!("{:>2} ", val);
        }
        println!();
    }

    // Compute the transpose
    let mut transpose: Vec<Vec<i32>> = vec![vec![0; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            transpose[j][i] = matrix[i][j];
        }
    }

    // Print the transposed matrix
    println!("\nTransposed Matrix:");
    for row in &transpose {
        for &val in row {
            print!("{:>2} ", val);
        }
        println!();
    }
}

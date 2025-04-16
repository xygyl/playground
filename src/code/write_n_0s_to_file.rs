use std::fs::File;
use std::io::Write;
use std::process::exit;
use text_io::read;

pub fn write_n_0s_to_file() {
    print!("Number of 0s: ");
    let size: usize = read!();
    println!();

    print!("Filename: ");
    let filename: String = read!();
    println!();

    let mut file = match File::create(&filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error creating file {}: {}", filename, e);
            exit(1);
        }
    };

    let zeros = vec![b'0'; size];
    if let Err(e) = file.write_all(&zeros) {
        eprintln!("Error writing to file {}: {}", filename, e);
        exit(1);
    }

    println!("File '{}' created with {} zeros.", filename, size);
}

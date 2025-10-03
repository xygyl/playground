use inquire::{CustomType, Text};
use std::{fs::File, io::Write, process::exit};

/// Generates a file with n 0s.
pub fn write_n_0s_to_file() {
    let size: usize = CustomType::new("Number of 0s:").prompt().unwrap();
    let filename = Text::new("Filename:").prompt().unwrap();
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

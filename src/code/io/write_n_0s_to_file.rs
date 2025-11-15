use inquire::{CustomType, Text};
use std::{fs::File, io::Write, process::exit};

const CHUNK_SIZE: usize = 1_000_000;

/// Generates a file with n 0s.
pub fn write_n_0s_to_file() -> Option<()> {
    let size: usize = CustomType::new("Number of 0s:").prompt().ok()?;
    let filename = Text::new("Filename:").prompt().ok()?;
    let mut file = match File::create(&filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error creating file {}: {}", filename, e);
            exit(1);
        }
    };

    let zeros = vec![b'0'; CHUNK_SIZE];
    let mut remaining = size;

    while 0 < remaining {
        let to_write = remaining.min(CHUNK_SIZE);
        if let Err(e) = file.write_all(&zeros) {
            eprintln!("Error writing to file {}: {}", filename, e);
            exit(1);
        }
        remaining -= to_write;
    }

    println!("File '{}' created with {} zeros.", filename, size);
    Some(())
}

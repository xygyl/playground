use num_format::{Locale, ToFormattedString};
use text_io::read;

/// Raises an array of [1..=10] to power of the given number.
pub fn squared_array() {
    // let bases: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    print!("Enter max: ");
    let max: u128 = read!();

    print!("\nEnter power: ");
    let p: u32 = read!();

    for e in 1..=max {
        println!(
            "{}^{} = {}",
            e,
            p,
            e.pow(p).to_formatted_string(&Locale::en)
        );
    }
}

use num_format::{Locale, ToFormattedString};
use text_io::read;

pub fn func_squared_array() {
    let bases: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    print!("\nEnter power: ");
    let p: u128 = read!();

    for e in bases {
        println!(
            "{}^{} = {}",
            p,
            e,
            p.pow(e).to_formatted_string(&Locale::en)
        );
    }
}

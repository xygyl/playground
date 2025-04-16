use num_format::{Locale, ToFormattedString};
use text_io::read;

pub fn collatz() {
    print!("\nPlease enter number to calculate: ");
    let mut n: u128 = read!();
    let mut sequence = vec![n];
    let mut iter = 0;

    while n != 1 {
        match n % 2 == 0 {
            true => n /= 2,
            false => n = (3 * n) + 1,
        }
        sequence.push(n);
        iter += 1;
    }

    let formatted_sequence: Vec<String> = sequence
        .iter()
        .map(|n| n.to_formatted_string(&Locale::en))
        .collect();

    println!(
        "Collatz sequence for {}: {:?}",
        n.to_formatted_string(&Locale::en),
        formatted_sequence.join(" -> ")
    );
    println!("\n{} iterations", iter);
}

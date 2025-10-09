use inquire::CustomType;
use num_format::{Locale, ToFormattedString};

/// Prints the collatz sequence for a given number.
pub fn collatz() -> Option<()> {
    let mut n: u128 = CustomType::new("Enter n:")
        .with_help_message("Collatz sequence for the nth number")
        .prompt()
        .ok()?;
    let mut sequence = vec![n];
    let mut iter = 0;

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = (3 * n) + 1;
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
    Some(())
}

pub fn collatz_arg(mut n: u128) {
    let display_n = n;
    let mut sequence = vec![n];
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = (3 * n) + 1;
        }
        sequence.push(n);
    }
    let formatted_sequence: Vec<String> = sequence
        .iter()
        .map(|n| n.to_formatted_string(&Locale::en))
        .collect();

    println!(
        "Collatz sequence for {}:\n{:?}",
        display_n.to_formatted_string(&Locale::en),
        formatted_sequence.join(" -> ")
    );
}

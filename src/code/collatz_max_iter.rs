use dashmap::DashMap;
use inquire::{Confirm, CustomType};
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
use std::sync::Arc;

use crate::code::collatz::collatz_arg;

/// Gives the number with the most collatz iterations from a given range.
pub fn collatz_max_iter() {
    let n: u64 = CustomType::new("Enter n:")
        .with_help_message("The range of number to calculate (1..=input)")
        .prompt()
        .unwrap();
    let show = Confirm::new("Show collatz sequence of result?")
        .with_default(false)
        .prompt()
        .unwrap();
    let numbers: Vec<u64> = (1..=n).collect();

    // Use an Arc-wrapped DashMap for concurrent caching.
    let collatz_map = Arc::new(DashMap::new());

    let results: Vec<(u64, u64)> = numbers
        .par_iter()
        .map(|&num| {
            let mut n = num;
            let mut iter: u64 = 0;
            // Keep track of the intermediate values in this computation.
            let mut temp_seq = Vec::new();

            while n != 1 {
                // If we already computed n, use that cached value.
                if let Some(cached) = collatz_map.get(&n) {
                    iter += *cached;
                    break;
                }
                temp_seq.push(n);

                // Apply the Collatz formula.
                match n % 2 == 0 {
                    true => n /= 2,
                    false => n = (3 * n) + 1,
                }
                iter += 1;
            }

            // Now, update the cache with the computed values for this chain.
            let mut chain_iter = iter;
            for &x in &temp_seq {
                collatz_map.insert(x, chain_iter);
                // Decrease the count for the next element in the chain.
                chain_iter = chain_iter.saturating_sub(1);
            }

            (num, iter)
        })
        .collect();

    // Find the number with the maximum iterations.
    match results.iter().max_by_key(|(_num, iterations)| iterations) {
        Some(&(number_with_max_iter, max_iter)) => {
            println!(
                "\n{} took the most iterations: {}\n",
                number_with_max_iter.to_formatted_string(&Locale::en),
                max_iter.to_formatted_string(&Locale::en)
            );
            if show == true {
                collatz_arg(number_with_max_iter as u128);
            } 
        }
        None => println!("\nNo numbers processed."),
    }
}

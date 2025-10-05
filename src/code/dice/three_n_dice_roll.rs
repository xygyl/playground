use crate::helper::green_to_red_gradient::green_to_red_gradient;
use inquire::{Confirm, CustomType};
use num_format::{Locale, ToFormattedString};
use owo_colors::OwoColorize;
use rand::Rng;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};

/// Generates three random numbers between [0..=n] and ends when they're all the same.
pub fn three_n_dice_roll() {
    let show = Confirm::new("Show iterations?")
        .with_default(true)
        .with_help_message(
            "Whether to print each iteration or only show how many iterations it took",
        )
        .prompt()
        .unwrap();
    let n: u32 = CustomType::new("Enter n:")
        .with_help_message("Range for the roll (1..=input)")
        .prompt()
        .unwrap();
    let mut iter = 0;
    let now = std::time::Instant::now();

    loop {
        let vals: [u32; 3] = (0..3)
            .into_par_iter()
            .map(|_| rand::rng().random_range(0..=n))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let all_equal = vals.par_iter().skip(1).all(|v| *v == vals[0]);

        iter += 1;

        if show {
            if all_equal {
                println!(
                    "{} {} {}",
                    format!("{:2}", vals[0]).truecolor(0, 127, 255).to_string(),
                    format!("{:2}", vals[1]).truecolor(0, 127, 255).to_string(),
                    format!("{:2}", vals[2]).truecolor(0, 127, 255).to_string(),
                );
            } else {
                println!(
                    "{} {} {}",
                    green_to_red_gradient(vals[0], n),
                    green_to_red_gradient(vals[1], n),
                    green_to_red_gradient(vals[2], n),
                );
            }
        }
        if all_equal {
            println!(
                "{} iterations in {}ms",
                iter.to_formatted_string(&Locale::en),
                now.elapsed().as_millis()
            );
            break;
        }
    }
}

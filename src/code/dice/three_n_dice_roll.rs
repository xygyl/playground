use crate::helper::green_to_red_gradient::green_to_red_gradient;
use inquire::{Confirm, CustomType};
use num_format::{Locale, ToFormattedString};
use owo_colors::OwoColorize;
use rand::Rng;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};

/// Generates three random numbers between [0..=n] and ends when they're all the same.
pub fn three_n_dice_roll() -> Option<()> {
    let show = Confirm::new("Show iterations?")
        .with_default(true)
        .with_help_message(
            "Whether to print each iteration or only show how many iterations it took",
        )
        .prompt()
        .ok()?;
    let n: u32 = CustomType::new("Enter n:")
        .with_help_message("Range for the roll (1..=input)")
        .prompt()
        .ok()?;
    let mut iter = 0;

    loop {
        let vals: Vec<u32> = (0..3)
            .into_par_iter()
            .map(|_| rand::rng().random_range(0..=n))
            .collect::<Vec<_>>();
        let all_equal = vals.par_iter().skip(1).all(|&v| v == vals[0]);

        iter += 1;

        if show {
            if all_equal {
                println!(
                    "{}",
                    vals[..3]
                        .iter()
                        .map(|v| format!("{:>2}", v).truecolor(0, 127, 255).to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            } else {
                println!(
                    "{}",
                    vals[..3]
                        .iter()
                        .map(|v| green_to_red_gradient(*v, n))
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
        }
        if all_equal {
            println!("{} iterations", iter.to_formatted_string(&Locale::en),);
            break;
        }
    }
    Some(())
}

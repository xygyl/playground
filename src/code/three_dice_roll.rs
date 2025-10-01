use inquire::{Confirm, CustomType};
use rand::Rng;

/// Generates three random numbers between [0..=n] and ends when they're all the same.
pub fn three_dice_roll() {
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

    loop {
        let r1 = rand::rng().random_range(0..=n);
        let r2 = rand::rng().random_range(0..=n);
        let r3 = rand::rng().random_range(0..=n);

        match r1 == r2 && r2 == r3 {
            true => {
                iter += 1;
                if show {
                    println!("{} {} {}", r1, r2, r3);
                }
                println!("{} iterations", iter);
                break;
            }
            false => {
                iter += 1;
                if show {
                    println!("{} {} {}", r1, r2, r3);
                }
            }
        }
    }
}

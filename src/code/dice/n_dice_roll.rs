use inquire::CustomType;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// Rolls a dice
pub fn n_dice_roll() -> Option<()> {
    let sides: u32 = CustomType::new("Enter number of sides:").prompt().ok()?;
    match sides {
        0 => return None,
        _ => {
            let roll_n: u32 = CustomType::new("Enter number of rolls:").prompt().ok()?;
            let rolls: String = (1..=roll_n)
                .into_par_iter()
                .map(|_| rand::rng().random_range(1..=sides).to_string())
                .reduce(|| String::new(), |mut a, b| {
                    if !a.is_empty() {
                        a.push(' ');
                    }
                    a.push_str(&b);
                    a
                });
            println!("{}", rolls);
            Some(())
        }
    }
}

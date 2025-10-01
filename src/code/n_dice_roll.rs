use inquire::CustomType;
use rand::Rng;

/// Rolls a dice
pub fn n_dice_roll() {
    loop {
        let sides: u8 = CustomType::new("Enter number of sides:").prompt().unwrap();
        match sides {
            0 => break,
            _ => {
                let dice = rand::rng().random_range(1..=sides);
                println!("{}", dice);
            }
        }
    }
}

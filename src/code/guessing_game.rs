use inquire::CustomType;
use rand::Rng;
use std::cmp::Ordering;

/// Number guessing game with a range from 1-200.
pub fn guessing_game() {
    let max: u32 = CustomType::new("Enter max:").prompt().unwrap();
    println!("Guess the number between 1 and {}!", max);

    let s_num = rand::rng().random_range(1..=max);

    loop {
        let guess: u32 = CustomType::new("Enter you guess:").prompt().unwrap();

        match guess.cmp(&s_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

use rand::Rng;
use std::cmp::Ordering;
use text_io::read;

/// Number guessing game with a range from 1-200.
pub fn guessing_game() {
    println!("\nGuess the number between 1 and 200!");

    let s_num = rand::rng().random_range(1..=200);

    loop {
        print!("Please input your guess: ");

        let guess: u8 = read!();

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

use rand::Rng;
use text_io::read;

/// Generates three random numbers between [0..=n] and ends when they're all the same.
pub fn three_dice_roll() {
    print!("\nPlease input max range: ");

    let i: u32 = read!();
    let mut iter = 0;

    loop {
        let r1 = rand::rng().random_range(0..=i);
        let r2 = rand::rng().random_range(0..=i);
        let r3 = rand::rng().random_range(0..=i);

        if r1 == r2 && r2 == r3 {
            println!("{} {} {}", r1, r2, r3);
            println!("{} iterations", iter);
            break;
        }
        println!("{} {} {}", r1, r2, r3);
        iter += 1;
    }
}

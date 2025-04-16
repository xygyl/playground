use rand::Rng;
use text_io::read;

pub fn n_dice_roll() {
    loop {
        print!("\nEnter number of die sides: ");
        let i: u8 = read!();
        if i <= 20 {
            let dice = rand::rng().random_range(1..=i);
            println!("{}", dice);
        }
        if i > 20 {
            break;
        }
    }
}

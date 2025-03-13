use rand::Rng;
use text_io::read;

pub fn func_three_dice_roll() {
    print!("\nPlease input max range: ");

    let i: u8 = read!();

    loop {
        let r1 = rand::rng().random_range(0..=i);
        let r2 = rand::rng().random_range(0..=i);
        let r3 = rand::rng().random_range(0..=i);

        if r1 == r2 && r2 == r3 {
            println!("{}, {}, {}", r1, r2, r3);
            break;
        }
    }
}

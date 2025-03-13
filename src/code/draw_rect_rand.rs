use rand::Rng;
use text_io::read;

pub fn func_draw_rect_rand() {
    print!("\nEnter min value: ");
    let min: u8 = read!();

    print!("Enter max value: ");
    let max: u8 = read!();

    print!("Enter character: ");
    let ctr: String = read!();
    println!("");

    let width = rand::rng().random_range(min..=max);
    let height = rand::rng().random_range(min..=max);

    for _ in 0..height {
        let r = ctr.repeat(width as usize);
        println!("{}", r);
    }

    println!("");
    println!("Width: {}", width);
    println!("Height: {}", height);

    if width == height {
        print!("It's a square!")
    }
}

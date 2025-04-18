use text_io::read;

/// Prints a rectangle given a width, height, and a character.
pub fn draw_rect() {
    print!("\nEnter rectangle width: ");
    let width: u8 = read!();

    print!("Enter rectangle height: ");
    let height: u8 = read!();

    print!("Enter rectangle character: ");
    let ctr: String = read!();

    for _ in 0..height {
        let r = ctr.repeat(width as usize);
        println!("{}", r);
    }
}

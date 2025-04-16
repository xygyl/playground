use text_io::read;

pub fn is_even_odd() {
    loop {
        print!("\nPlease input number: ");
        let i: i128 = read!();
        match i % 2 {
            0 => println!("{} is even.", i),
            -1 | 1 => println!("{} is odd.", i),
            _ => println!("{} is not an integer.", i),
        }
    }
}

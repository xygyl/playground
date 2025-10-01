use inquire::CustomType;

/// Returns whether a number is even or odd.
pub fn is_even_odd() {
    loop {
        let i: i128 = CustomType::new("Input n:")
            .with_help_message("Determine whether n is even or odd")
            .prompt()
            .unwrap();
        match i {
            0 => println!("0 is neither even nor odd."),
            _ => match i % 2 {
                0 => println!("{} is even.", i),
                -1 | 1 => println!("{} is odd.", i),
                _ => println!("{} is not an integer.", i),
            },
        }
    }
}

use inquire::{CustomType, Text};

/// Prints a rectangle given a width, height, and a character.
pub fn draw_rect() {
    let width: u8 = CustomType::new("Enter width:").prompt().unwrap();
    let height: u8 = CustomType::new("Enter height:").prompt().unwrap();
    let ctr = Text::new("Enter rectangle character:").prompt().unwrap();

    for _ in 0..height {
        let r = ctr.repeat(width as usize);
        println!("{}", r);
    }
}

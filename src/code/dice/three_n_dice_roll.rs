use inquire::{Confirm, CustomType};
use num_format::{Locale, ToFormattedString};
use owo_colors::OwoColorize;
use rand::Rng;

/// Generates three random numbers between [0..=n] and ends when they're all the same.
pub fn three_n_dice_roll() {
    let show = Confirm::new("Show iterations?")
        .with_default(true)
        .with_help_message(
            "Whether to print each iteration or only show how many iterations it took",
        )
        .prompt()
        .unwrap();
    let n: u32 = CustomType::new("Enter n:")
        .with_help_message("Range for the roll (1..=input)")
        .prompt()
        .unwrap();
    let mut iter = 0;

    loop {
        let r1 = rand::rng().random_range(0..=n);
        let r2 = rand::rng().random_range(0..=n);
        let r3 = rand::rng().random_range(0..=n);
        if r1 == r2 && r2 == r3 {
            iter += 1;
            if show {
                println!(
                    "{} {} {}",
                    format!("{:2}", r1).truecolor(0, 127, 255).to_string(),
                    format!("{:2}", r2).truecolor(0, 127, 255).to_string(),
                    format!("{:2}", r3).truecolor(0, 127, 255).to_string(),
                );
            }
            println!("{} iterations", iter.to_formatted_string(&Locale::en));
            break;
        } else {
            iter += 1;
            if show {
                println!(
                    "{} {} {}",
                    gradient_color(n, r1),
                    gradient_color(n, r2),
                    gradient_color(n, r3),
                );
            }
        }
    }
}

fn gradient_color(max: u32, val: u32) -> impl std::fmt::Display {
    // Normalize to [0.0, 1.0]
    let t = val as f32 / max as f32;

    // Interpolate:
    // green (0,255,0) → yellow (255,255,0) at t=0.5 → red (255,0,0) at t=1
    let (r, g, b) = if t < 0.5 {
        // green → yellow
        let f = t / 0.5;
        let r = (255.0 * f) as u8;
        let g = 255;
        let b = 0;
        (r, g, b)
    } else {
        // yellow → red
        let f = (t - 0.5) / 0.5;
        let r = 255;
        let g = (255.0 * (1.0 - f)) as u8;
        let b = 0;
        (r, g, b)
    };

    let s = format!("{:2}", val);
    s.truecolor(r, g, b).to_string()
}

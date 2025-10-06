use inquire::CustomType;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

pub fn stupid_even_odd() -> Option<()> {
    let n: u32 = CustomType::new("Please enter max for range:")
        .prompt()
        .ok()?;
    let file = File::create("stupid.rs").ok()?;
    let mut w = BufWriter::new(file);

    writeln!(w, "fn main() {{").ok()?;
    writeln!(w, "    let n = {};", n).ok()?;
    writeln!(w, "    match n {{").ok()?;

    for i in 0..=n {
        match i % 2 {
            0 => writeln!(w, r#"        {} => println!("{} is even"),"#, i, i).ok()?,
            _ => writeln!(w, r#"        {} => println!("{} is odd"),"#, i, i).ok()?,
        }
    }

    writeln!(w, "    }}").ok()?;
    writeln!(w, "}}").ok()?;

    Some(())
}

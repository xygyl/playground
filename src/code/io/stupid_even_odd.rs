use inquire::CustomType;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

pub fn stupid_even_odd() -> Option<()> {
    let n: u32 = CustomType::new("Max for range:")
        .with_help_message("0..=n")
        .prompt()
        .ok()?;
    let mut w = BufWriter::new(File::create("stupid.rs").ok()?);

    writeln!(w, "fn main() {{").ok()?;
    writeln!(w, "    let n = {};", n).ok()?;
    writeln!(w, "    match n {{").ok()?;

    for i in 0..=n {
        match i % 2 {
            0 => writeln!(w, r#"        {} => println!("{} is even"),"#, i, i).ok()?,
            _ => writeln!(w, r#"        {} => println!("{} is odd"),"#, i, i).ok()?,
        }
    }

    writeln!(w, r#"        _ => println!("kys"),"#).ok()?;
    writeln!(w, "    }}").ok()?;
    writeln!(w, "}}").ok()?;

    Some(())
}

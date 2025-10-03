use inquire::CustomType;
use std::{
    fs::File,
    io::{BufWriter, Result, Write},
};

pub fn stupid_even_odd() -> Result<()> {
    let n: u32 = CustomType::new("Please enter max for range:")
        .prompt()
        .unwrap();
    let file = File::create("stupid.rs")?;
    let mut w = BufWriter::new(file);

    writeln!(w, "fn main() {{")?;
    writeln!(w, "    let n = {};", n)?;
    writeln!(w, "    match n {{")?;

    for i in 0..=n {
        match i % 2 {
            0 => writeln!(w, r#"        {} => println!("{} is even"),"#, i, i)?,
            _ => writeln!(w, r#"        {} => println!("{} is odd"),"#, i, i)?,
        }
    }

    writeln!(w, "    }}")?;
    writeln!(w, "}}")?;

    Ok(())
}

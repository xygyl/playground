use std::fs::File;
use std::io::{BufWriter, Result, Write};
use text_io::read;

pub fn stupid_even_odd() -> Result<()> {
    print!("\nPlease enter max for range: ");
    let n: u32 = read!();

    let file = File::create("stupid.rs")?;
    let mut w = BufWriter::new(file);

    writeln!(w, "fn main() {{")?;
    writeln!(w, "    let n = {};", n)?;
    writeln!(w, "    match n {{")?;

    for i in 0..=n {
        if i % 2 == 0 {
            writeln!(w, r#"        {} => println!("{} is even"),"#, i, i)?;
        } else {
            writeln!(w, r#"        {} => println!("{} is odd"),"#, i, i)?;
        }
    }

    writeln!(w, "    }}")?;
    writeln!(w, "}}")?;

    Ok(())
}

/* use text_io::read;

pub fn stupid_even_odd() {
    print!("\nPlease enter max for range: ");
    let n: u32 = read!();
    println!();
    println!("fn main() {{");
    println!("    let n = {};", n);
    println!("    match n {{");
    for i in 0..=n {
        if i % 2 == 0 {
            println!(r#"        {} => println!("{} is even"),"#, i, i);
        } else {
            println!(r#"        {} => println!("{} is odd"),"#, i, i);
        }
    }
    println!("    }}");
    println!("}}");
} */

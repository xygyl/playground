use std::fs::File;
use std::io::{BufWriter, Write};
use text_io::read;

pub fn stupid_even_odd() {
    print!("\nPlease enter max for range: ");
    std::io::stdout().flush().unwrap();
    let n: u32 = read!();

    let file = File::create("stupid.rs").unwrap();
    let mut w = BufWriter::new(file);

    writeln!(w, "fn main() {{").unwrap();
    writeln!(w, "    let n = {};", n).unwrap();
    writeln!(w, "    match n {{").unwrap();

    for i in 0..=n {
        if i % 2 == 0 {
            writeln!(w, r#"        {} => println!("{} is even"),"#, i, i).unwrap();
        } else {
            writeln!(w, r#"        {} => println!("{} is odd"),"#, i, i).unwrap();
        }
    }

    writeln!(w, "    }}").unwrap();
    writeln!(w, "}}").unwrap();
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

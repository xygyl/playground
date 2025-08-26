use std::{thread, time};

pub fn death_clock() {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;

    loop {
        if a == 100 {
            a = 0;
            b += 1;
        }
        if b == 100 {
            b = 0;
            c += 1;
        }
        if c == 100 {
            c = 0;
            d += 1;
        }
        if d == 100 {
            println!("{d}");
            break;
        }
        a += 1;
        print!("{}[2J", 27 as char);
        println!("{a}");
        println!("{b}");
        println!("{c}");
        println!("{d}");
        thread::sleep(time::Duration::from_nanos(1));
    }
}

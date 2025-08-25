use num_bigint::BigUint;
use num_traits::{One};
use text_io::read;

pub fn factorial() {
    print!("Please enter number: ");
    let num: u32 = read!();
    let mut result: BigUint = One::one();
    for i in 1..=num {
        result *= i;
    }
    println!("{}", result);
}

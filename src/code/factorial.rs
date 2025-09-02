use num_bigint::BigUint;
use num_traits::{One};
use text_io::read;

pub fn factorial() {
    print!("Please enter number: ");
    let num: u128 = read!();
    let result: BigUint = (1..=num)
        .fold(
            BigUint::one(),
            |acc, x| acc * x
        );
    println!("{}! = {}", num, result);
}

use num_bigint::BigUint;
use num_traits::{One};
use text_io::read;

pub fn factorial() {
    print!("Please enter number: ");
    let num: u128 = read!();
    let mut vec: Vec<u128> = Vec::new();
    for i in 1..=num {
        vec.push(i);
    }
    let result: BigUint = vec
        .iter()
        .fold(
            BigUint::one(),
            |acc, x| acc * x
        );
    println!("{}! = {}", num, result);
}

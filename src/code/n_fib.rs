use num_bigint::BigUint;
use num_traits::{One, Zero};
use text_io::read;

pub fn get_nth_fib_num(n: u128) -> BigUint {
    if n == 0 {
        return BigUint::zero();
    } else if n == 1 {
        return BigUint::one();
    }

    let mut prev_prev = BigUint::zero();
    let mut prev = BigUint::one();
    let mut current = BigUint::zero();

    for _ in 2..=n {
        current = &prev + &prev_prev;
        prev_prev = prev;
        prev = current.clone();
    }

    current
}

pub fn main() {
    print!("Enter n: ");
    let n: u128 = read!();
    println!("The {}th Fibonacci number is: {}", n, get_nth_fib_num(n));
}

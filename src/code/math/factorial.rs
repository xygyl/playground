use inquire::CustomType;
use num_bigint::BigUint;
// use num_traits::One;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::ops::Mul;

pub fn factorial() -> Option<()> {
    let num: u32 = CustomType::new("Enter n:")
        .with_help_message("Calculate the factorial of n")
        .prompt()
        .ok()?;

    // let result: BigUint = (1..=num).fold(BigUint::one(), |acc, x| acc * x);
    // let result: BigUint = (1..=num).map(BigUint::from).fold(BigUint::one(), Mul::mul);
    let result: BigUint = (1..=num)
        .into_par_iter()
        .map(BigUint::from)
        .reduce_with(Mul::mul)
        .unwrap();
    println!("{}! = {}", num, result);
    Some(())
}

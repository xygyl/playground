use inquire::CustomType;
use num_bigint::BigUint;
use num_traits::One;

pub fn factorial() {
    let num: u128 = CustomType::new("Enter n:")
        .with_help_message("Calculate the factorial of n")
        .prompt()
        .unwrap();

    let result: BigUint = (1..=num).fold(BigUint::one(), |acc, x| acc * x);
    println!("{}! = {}", num, result);
}

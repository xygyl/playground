use inquire::CustomType;
use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Returns the nth fibonacci number.
fn fib(n: u128) -> BigUint {
    (0..n)
        .fold((BigUint::zero(), BigUint::one()), |(res, prev), _| {
            (res.clone() + prev, res)
        })
        .0
}

/// Runs the fibonacci function.
pub fn n_fib() -> Option<()> {
    let n: u128 = CustomType::new("Enter n:")
        .with_help_message("The nth fibonacci number")
        .prompt()
        .ok()?;
    let nf = fib(n);
    match n {
        1 => println!("The {}st Fibonacci number is: {}", n, nf),
        2 => println!("The {}nd Fibonacci number is: {}", n, nf),
        3 => println!("The {}rd Fibonacci number is: {}", n, nf),
        _ => println!("The {}th Fibonacci number is: {}", n, nf),
    }
    Some(())
}

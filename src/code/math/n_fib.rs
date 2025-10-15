use inquire::CustomType;
use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Returns the nth fibonacci number.
// fn fib(n: u128) -> BigUint {
//     (0..n).fold((BigUint::zero(), BigUint::one()), |(res, prev), _| { (res.clone() + prev, res) }).0
// }

fn fib2(n: u64) -> (BigUint, BigUint) {
    if n == 0 {
        return (BigUint::zero(), BigUint::one());
    }
    let (a, b) = fib2(n >> 1); // a = F(k), b = F(k+1)
                               // c = F(2k) = F(k) * [2*F(k+1) − F(k)]
    let two_a = &a << 1;
    let b_minus_a = if b >= a { &b - &a } else { BigUint::zero() }; // (safe: b>=a always holds)
    let c = &a * (two_a + b_minus_a);
    // d = F(2k+1) = F(k)^2 + F(k+1)^2
    let d = &a * &a + &b * &b;

    if n & 1 == 0 {
        (c, d) // (F(2k), F(2k+1))
    } else {
        (d.clone(), c + d) // (F(2k+1), F(2k)+F(2k+1))
    }
}

pub fn fib_fast(n: u64) -> BigUint {
    fib2(n).0
}

/// Runs the fibonacci function.
pub fn n_fib() -> Option<()> {
    let n: u64 = CustomType::new("Enter n:")
        .with_help_message("The nth fibonacci number")
        .prompt()
        .ok()?;
    let nf = fib_fast(n);
    match n {
        1 => println!("The {}st Fibonacci number is: {}", n, nf),
        2 => println!("The {}nd Fibonacci number is: {}", n, nf),
        3 => println!("The {}rd Fibonacci number is: {}", n, nf),
        _ => println!("The {}th Fibonacci number is: {}", n, nf),
    }
    Some(())
}

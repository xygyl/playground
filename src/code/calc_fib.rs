use num_bigint::BigUint;
use num_traits::{One, Zero};
use text_io::read;

/// Returns the nth fibonacci number.
// pub fn n_fib(n: u128) -> BigUint {
//     match n {
//         0 => return BigUint::zero(),
//         1 => return BigUint::one(),
//         _ => {
//             let mut prev_prev = BigUint::zero();
//             let mut prev = BigUint::one();
//             let mut current = BigUint::zero();

//             for _ in 2..=n {
//                 current = &prev + &prev_prev;
//                 prev_prev = prev;
//                 prev = current.clone();
//             }

//             current
//         }
//     }
// }

/// Returns the nth fibonacci number.
fn n_fib(n: u128) -> BigUint {
    (0..n)
        .fold(
            (BigUint::zero(), BigUint::one()),
            |(res, prev), _| 
            (res.clone() + prev, res)
        ).0
}

/// Runs the fibonacci function.
pub fn calc_fib() {
    print!("Enter n: ");
    let n: u128 = read!();
    let nf = n_fib(n);
    match n {
        1 => println!("The {}st Fibonacci number is: {}", n, nf),
        2 => println!("The {}nd Fibonacci number is: {}", n, nf),
        3 => println!("The {}rd Fibonacci number is: {}", n, nf),
        _ => println!("The {}th Fibonacci number is: {}", n, nf),
    }
}

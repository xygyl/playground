use inquire::CustomType;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::hint::black_box;

pub fn collatz_to_n() -> Option<()> {
    let n: u128 = CustomType::new("Enter n:")
        .with_help_message("Checks if every number in 2..=n terminates.")
        .prompt()
        .ok()?;
    (2..=n).into_par_iter().for_each(|v| {
        let mut var = v;
        while var != 1 {
            var = if var % 2 == 0 {
                var / 2
            } else {
                (3 * var) + 1
            };
            black_box(var);
        }
    });
    Some(())
}

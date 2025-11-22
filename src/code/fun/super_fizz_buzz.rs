use inquire::{CustomType, InquireError};
use std::{fmt::Display, ops::Rem};

fn fb_calc<T>(number: T, d_nums: &[(T, &str)]) -> String
where
    T: Copy + Display + From<u8> + PartialEq + Rem<Output = T>,
{
    Some(
        d_nums
            .iter()
            .filter_map(|(n, s)| (number % *n == T::from(0)).then_some(*s))
            .collect::<String>(),
    )
    .filter(|s| !s.is_empty())
    .map_or(number.to_string(), |s| format!("{}: {}", number, s))
}

pub fn super_fizz_buzz() -> Option<()> {
    let rules: u32 = CustomType::new("How many rules?").prompt().ok()?;
    let r: Vec<(u32, String)> = (0..rules)
        .map(|_| -> Result<_, InquireError> {
            Ok((
                CustomType::<u32>::new("Enter number:").prompt()?,
                CustomType::<String>::new("Enter string:").prompt()?,
            ))
        })
        .collect::<Result<Vec<(u32, String)>, InquireError>>()
        .ok()?;
    let refs: Vec<(u32, &str)> = r.iter().map(|(n, s)| (*n, s.as_str())).collect();
    let iter: u32 = CustomType::new("Range:")
        .with_help_message("1..=n")
        .with_default(100)
        .prompt()
        .ok()?;

    for i in 1..=iter {
        println!("{}", fb_calc(i, &refs))
    }

    Some(())
}

/* fn fb_calc(number: u32, d_nums: &Vec<(u32, &str)>) -> String {
    let mut res: String = String::new();
    let mut bul: Vec<bool> = Vec::new();

    d_nums.iter().for_each(|(n, _)| {
        bul.push(number % n == 0);
    });

    if bul.iter().all(|&x| x == false) {
        number.to_string()
    } else {
        bul.iter().enumerate().for_each(|(i, &b)| {
            if b {
                res.push_str(d_nums[i].1);
            }
        });
        format!("{}: {}", number, res)
    }
}

fn main() {
    let rules: Vec<(u32, &str)> = vec![
        (3, "Three"),
        (5, "Five"),
        (7, "Seven"),
        (9, "Nine"),
        (11, "Eleven"),
    ];

    (1..=100)
        .map(|i| fb_calc(i, &rules))
        .for_each(|i| println!("{}", i));
} */

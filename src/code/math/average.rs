use inquire::Text;

fn calculate_average(nums: &[f64]) -> Option<f64> {
    let sum: f64 = nums.iter().sum();
    if sum.is_nan() || sum.is_infinite() || nums.is_empty() {
        return None;
    } else {
        return Some(sum / nums.len() as f64);
    }
}

pub fn input_average() -> Option<()> {
    let avg = calculate_average(
        &Text::new("Enter numbers:")
            .prompt()
            .ok()?
            .split_whitespace()
            .filter_map(|v| v.parse::<f64>().ok())
            .collect::<Vec<f64>>(),
    );
    match avg {
        Some(val) => println!("\n{}", val),
        None => println!("\nError!"),
    }
    Some(())
}

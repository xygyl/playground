use inquire::CustomType;

pub fn percent_off() -> Option<()> {
    let price: f64 = CustomType::new("Enter item price:").prompt().ok()?;
    let discount: f64 = CustomType::new("Enter percent discount:").prompt().ok()?;
    let final_price = match discount {
        0.0 => price,
        _ => price - (price * (discount * 0.01)),
    };
    println!("${:.2}", final_price);
    Some(())
}

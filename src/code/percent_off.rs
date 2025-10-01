use inquire::CustomType;

pub fn percent_off() {
    let price: f64 = CustomType::new("Enter item price:").prompt().unwrap();
    let discount: f64 = CustomType::new("Enter percent discount:").prompt().unwrap();
    let final_price = price - (price * (discount * 0.01));
    println!("${:.2}", final_price);
}

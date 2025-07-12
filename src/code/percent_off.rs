use text_io::read;

pub fn percent_off() {
    print!("Enter item price:\n");
    let price: f32 = read!();
    print!("Enter percent discount:\n");
    let discount: f32 = read!();
    let final_price = price - (price * (discount * 0.01));
    println!("${} with {}% discount is ${}", price, discount, final_price);
}

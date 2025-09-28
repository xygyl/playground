use inquire::Select;

mod code;

fn main() {
    let options = vec![
        "exit",
        "array_transpose",
        "collatz",
        "collatz_max_iter",
        "death_clock",
        "draw_rect",
        "draw_rect_rand",
        "factorial",
        "guessing_game",
        "is_even_odd",
        "most_frequent_letter",
        "n_dice_roll",
        "n_fib",
        "percent_off",
        "squared_array",
        "stupid_even_odd",
        "three_dice_roll",
        "write_n_zeroes_to_file",
    ];

    let result = Select::new("Select function", options.clone())
        .prompt()
        .unwrap();

    loop {
        match result {
            "exit" => break,
            "guessing_game" => code::guessing_game::guessing_game(),
            "three_dice_roll" => code::three_dice_roll::three_dice_roll(),
            "n_dice_roll" => code::n_dice_roll::n_dice_roll(),
            "draw_rect" => code::draw_rect::draw_rect(),
            "draw_rect_rand" => code::draw_rect_rand::draw_rect_rand(),
            "squared_array" => code::squared_array::squared_array(),
            "is_even_odd" => code::is_even_odd::is_even_odd(),
            "collatz" => code::collatz::collatz(),
            "collatz_max_iter" => code::collatz_max_iter::collatz_max_iter(),
            "write_n_zeroes_to_file" => code::write_n_0s_to_file::write_n_0s_to_file(),
            "most_frequent_letter" => code::most_frequent_letter::most_frequent_letter(),
            "array_transpose" => code::array_transpose::array_transpose(),
            "n_fib" => code::calc_fib::calc_fib(),
            "stupid_even_odd" => code::stupid_even_odd::stupid_even_odd().expect("bruh"),
            "percent_off" => code::percent_off::percent_off(),
            "factorial" => code::factorial::factorial(),
            "death_clock" => code::death_clock::death_clock(),
            _ => println!("\nPLEASE ENTER A NUMBER BETWEEN 0 AND 17 INCLUSIVE"), 
        }
        break;
    }
}

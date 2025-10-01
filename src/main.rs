use inquire::Select;
use terminal_size::terminal_size;

mod code;

fn make_border(w: u16) -> String {
    format!("├{}┤", "─".repeat(w as usize - 2))
}

fn main() {
    let width = terminal_size().unwrap().0.0;
    let border = make_border(width);
    let options = vec![
        "exit",
        "array_transpose",
        "collatz",
        "collatz_max_iter",
        "death_clock",
        "draw_rect",
        "factorial",
        "guessing_game",
        "is_even_odd",
        "matrix_average",
        "most_frequent_letter",
        "n_dice_roll",
        "n_fib",
        "percent_off",
        "stupid_even_odd",
        "three_dice_roll",
        "write_n_zeroes_to_file",
    ];

    loop {
        let result = Select::new("Select function", options.clone())
            .prompt()
            .unwrap();
        match result {
            "exit" => break,
            "array_transpose" => code::array_transpose::array_transpose(),
            "collatz" => code::collatz::collatz(),
            "collatz_max_iter" => code::collatz_max_iter::collatz_max_iter(),
            "death_clock" => code::death_clock::death_clock(),
            "draw_rect" => code::draw_rect::draw_rect(),
            "factorial" => code::factorial::factorial(),
            "guessing_game" => code::guessing_game::guessing_game(),
            "is_even_odd" => code::is_even_odd::is_even_odd(),
            "matrix_average" => code::matrix_average::matrix_average(),
            "most_frequent_letter" => code::most_frequent_letter::most_frequent_letter(),
            "n_dice_roll" => code::n_dice_roll::n_dice_roll(),
            "n_fib" => code::n_fib::n_fib(),
            "percent_off" => code::percent_off::percent_off(),
            "stupid_even_odd" => code::stupid_even_odd::stupid_even_odd().expect("bruh"),
            "three_dice_roll" => code::three_dice_roll::three_dice_roll(),
            "write_n_zeroes_to_file" => code::write_n_0s_to_file::write_n_0s_to_file(),
            _ => eprintln!("NOT AN OPTION!"),
        }
        println!("\n{border}\n");
    }
}

use text_io::read;
mod code;

fn main() {
    loop {
        println!("\n0 -> exit");
        println!("1 -> guessing_game");
        println!("2 -> three_dice_roll");
        println!("3 -> n_dice_roll");
        println!("4 -> draw_rect");
        println!("5 -> draw_rect_rand");
        println!("6 -> squared_array");
        println!("7 -> is_even_odd");
        println!("8 -> collatz");
        println!("9 -> collatz max iter");
        println!("10 -> Write n zeroes to file");
        println!("11 -> most_frequent_letter");
        println!("12 -> array_transpose");
        println!("13 -> n_fib");
        println!("14 -> stupid_even_odd");
        println!("15 -> percent_off");
        println!("16 -> factorial");
        println!("17 -> death_clock");
        print!("Function selection: ");

        let c: u8 = read!();
        println!();

        match c {
            0 => break,
            1 => code::guessing_game::guessing_game(),
            2 => code::three_dice_roll::three_dice_roll(),
            3 => code::n_dice_roll::n_dice_roll(),
            4 => code::draw_rect::draw_rect(),
            5 => code::draw_rect_rand::draw_rect_rand(),
            6 => code::squared_array::squared_array(),
            7 => code::is_even_odd::is_even_odd(),
            8 => code::collatz::collatz(),
            9 => code::collatz_max_iter::collatz_max_iter(),
            10 => code::write_n_0s_to_file::write_n_0s_to_file(),
            11 => code::most_frequent_letter::most_frequent_letter(),
            12 => code::array_transpose::array_transpose(),
            13 => code::calc_fib::calc_fib(),
            14 => code::stupid_even_odd::stupid_even_odd().expect("bruh"),
            15 => code::percent_off::percent_off(),
            16 => code::factorial::factorial(),
            17 => code::death_clock::death_clock(),
            _ => println!("\nPLEASE ENTER A NUMBER BETWEEN 0 AND 17 INCLUSIVE"),
        }
    }
}

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
        print!("Function selection: ");

        let c: u8 = read!();

        match c {
            0 => break,
            1 => code::guessing_game::func_guessing_game(),
            2 => code::three_dice_roll::func_three_dice_roll(),
            3 => code::n_dice_roll::func_n_dice_roll(),
            4 => code::draw_rect::func_draw_rect(),
            5 => code::draw_rect_rand::func_draw_rect_rand(),
            6 => code::squared_array::func_squared_array(),
            7 => code::is_even_odd::func_is_even_odd(),
            8 => code::collatz::func_collatz(),
            9 => code::collatz_max_iter::collatz_max_iter(),
            10 => code::write_n_0s_to_file::func_write_n_0s_to_file(),
            11 => code::most_frequent_letter::find_most_frequent_letter(),
            12 => code::array_transpose::make_array_transpose(),
            13 => code::n_fib::main(),
            _ => println!("\nPLEASE ENTER 1-12"),
        }
    }
}

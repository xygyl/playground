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
        println!("9 -> Write n zeroes to file");
        println!("10 -> most_frequent_letter");
        println!("11 -> array_transpose");
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
            9 => code::write_n_0s_to_file::func_write_n_0s_to_file(),
            10 => code::most_frequent_letter::find_most_frequent_letter(),
            11 => code::array_transpose::make_array_transpose(),
            _ => println!("\nPLEASE ENTER 1-11"),
        }
    }
}

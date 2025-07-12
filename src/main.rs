use text_io::read;

mod code;
use crate::code::{
    array_transpose::array_transpose, collatz::collatz, collatz_max_iter::collatz_max_iter,
    draw_rect::draw_rect, draw_rect_rand::draw_rect_rand, guessing_game::guessing_game,
    is_even_odd::is_even_odd, most_frequent_letter::most_frequent_letter, n_dice_roll::n_dice_roll,
    n_fib::calc_fib, percent_off::percent_off, squared_array::squared_array,
    stupid_even_odd::stupid_even_odd, three_dice_roll::three_dice_roll,
    write_n_0s_to_file::write_n_0s_to_file,
};

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
        print!("Function selection: ");

        let c: u8 = read!();

        match c {
            0 => break,
            1 => guessing_game(),
            2 => three_dice_roll(),
            3 => n_dice_roll(),
            4 => draw_rect(),
            5 => draw_rect_rand(),
            6 => squared_array(),
            7 => is_even_odd(),
            8 => collatz(),
            9 => collatz_max_iter(),
            10 => write_n_0s_to_file(),
            11 => most_frequent_letter(),
            12 => array_transpose(),
            13 => calc_fib(),
            14 => stupid_even_odd(),
            15 => percent_off(),
            _ => println!("\nPLEASE ENTER 0-13"),
        }
    }
}

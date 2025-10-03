use inquire::Select;
use std::process::exit;
use strum_macros::{Display, VariantArray};
use strum::VariantArray;
use terminal_size::terminal_size;

mod code;

use code::{
    dice::{n_dice_roll::n_dice_roll, three_n_dice_roll::three_n_dice_roll},
    guessing_game::guessing_game,
    io::{stupid_even_odd::stupid_even_odd, write_n_0s_to_file::write_n_0s_to_file},
    leetcode::most_frequent_letter::most_frequent_letter,
    math::{
        collatz::{collatz::collatz, collatz_max_iter::collatz_max_iter},
        factorial::factorial,
        n_fib::n_fib,
    },
    matrices::{matrix_average::matrix_average, matrix_transpose::matrix_transpose},
    misc::{is_even_odd::is_even_odd, percent_off::percent_off},
};

fn make_border(w: u16) -> String {
    format!("├{}┤", "─".repeat(w as usize - 2))
}

#[derive(Clone, Display, VariantArray)]
enum Functions {
    Collatz,
    CollatzMaxIter,
    Factorial,
    GuessingGame,
    IsEvenOdd,
    MatrixAverage,
    MatrixTranspose,
    MostFrequentLetter,
    NDiceRoll,
    NFib,
    PercentOff,
    StupidEvenOdd,
    ThreeDiceRoll,
    WriteNZeroesToFile,
}

fn main() {
    let width = terminal_size().unwrap().0 .0;
    let border = make_border(width);
    loop {
        let result = Select::new("Select function", Functions::VARIANTS.to_vec())
            .with_page_size(Functions::VARIANTS.to_vec().len())
            .prompt()
            .unwrap_or_else(|_| exit(0));
        match result {
            Functions::Collatz => collatz(),
            Functions::CollatzMaxIter => collatz_max_iter(),
            Functions::Factorial => factorial(),
            Functions::GuessingGame => guessing_game(),
            Functions::IsEvenOdd => is_even_odd(),
            Functions::MatrixAverage => matrix_average(),
            Functions::MatrixTranspose => matrix_transpose(),
            Functions::MostFrequentLetter => most_frequent_letter(),
            Functions::NDiceRoll => n_dice_roll(),
            Functions::NFib => n_fib(),
            Functions::PercentOff => percent_off(),
            Functions::StupidEvenOdd => stupid_even_odd().expect("bruh"),
            Functions::ThreeDiceRoll => three_n_dice_roll(),
            Functions::WriteNZeroesToFile => write_n_0s_to_file(),
        }
        println!("\n{border}\n");
    }
}

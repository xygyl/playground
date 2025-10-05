use inquire::Select;
use owo_colors::OwoColorize;
use std::process::exit;
use strum::VariantArray;
use strum_macros::{Display, VariantArray};
use terminal_size::{terminal_size, Width};

mod code;
mod helper;

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

#[derive(Clone, Display, VariantArray)]
enum Functions {
    #[strum(to_string = "Collatz")]
    Collatz,
    #[strum(to_string = "Collatz Max Iter")]
    CollatzMaxIter,
    #[strum(to_string = "Factorial")]
    Factorial,
    #[strum(to_string = "Guassing Game")]
    GuessingGame,
    #[strum(to_string = "Is Even Odd")]
    IsEvenOdd,
    #[strum(to_string = "Matrix Average")]
    MatrixAverage,
    #[strum(to_string = "Matrix Transpose")]
    MatrixTranspose,
    #[strum(to_string = "Most Frequent Letter")]
    MostFrequentLetter,
    #[strum(to_string = "N Dice Roll")]
    NDiceRoll,
    #[strum(to_string = "N Fib")]
    NFib,
    #[strum(to_string = "Percent Off")]
    PercentOff,
    #[strum(to_string = "Stupid Even-Odd")]
    StupidEvenOdd,
    #[strum(to_string = "Three Dice Roll")]
    ThreeDiceRoll,
    #[strum(to_string = "Write N Zeroes to File")]
    WriteNZeroesToFile,
}

fn make_border(w: u16) -> String {
    format!("{}", "─".repeat(w as usize))
        .truecolor(107, 93, 255)
        .to_string()
}

fn main() {
    loop {
        let result = Select::new("Select function", Functions::VARIANTS.to_vec())
            .with_vim_mode(true)
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
        // Makes the border
        println!(
            "\n{}\n",
            make_border(match terminal_size() {
                Some((Width(h), _)) => h,
                None => 0,
            })
        );
    }
}

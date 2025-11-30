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
    fun::{
        link_cleaner::link_cleaner, most_frequent_letter::most_frequent_letter,
        super_fizz_buzz::super_fizz_buzz,
    },
    guessing_game::guessing_game,
    io::{stupid_even_odd::stupid_even_odd, write_n_0s_to_file::write_n_0s_to_file},
    math::{
        collatz::{
            collatz::collatz, collatz_max_iter::collatz_max_iter, collatz_to_n::collatz_to_n,
        },
        factorial::factorial,
        n_fib::n_fib,
    },
    matrices::{matrix_average::matrix_average, matrix_transpose::matrix_transpose},
    misc::{death_clock::run_death_clock, is_even_odd::is_even_odd, percent_off::percent_off},
};

macro_rules! check {
    ($expr:expr) => {
        match $expr {
            Some(()) => draw_border(107, 93, 255),
            None => {
                draw_border(107, 93, 255);
                continue;
            }
        }
    };
}

#[derive(Clone, Display, VariantArray)]
enum Functions {
    // Collatz,
    // CollatzMaxIter,
    // CollatzToN,
    // DeathClock
    // Factorial,
    // GuessingGame,
    // IsEvenOdd,
    // LinkCleaner,
    // MatrixAverage,
    // MatrixTranspose,
    // MostFrequentLetter,
    // NDiceRoll,
    // NFib,
    // PercentOff,
    // StupidEvenOdd,
    // SuperFizzBuzz,
    // ThreeDiceRoll,
    // WriteNZeroesToFile,
    #[strum(to_string = "Collatz")]
    Collatz,
    #[strum(to_string = "Collatz Max Iter")]
    CollatzMaxIter,
    #[strum(to_string = "Collatz to N")]
    CollatzToN,
    #[strum(to_string = "DeathClock")]
    DeathClock,
    #[strum(to_string = "Factorial")]
    Factorial,
    #[strum(to_string = "Guessing Game")]
    GuessingGame,
    #[strum(to_string = "Is Even Odd")]
    IsEvenOdd,
    #[strum(to_string = "Link Cleaner")]
    LinkCleaner,
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
    #[strum(to_string = "Super Fizz Buzz")]
    SuperFizzBuzz,
    #[strum(to_string = "Three Dice Roll")]
    ThreeDiceRoll,
    #[strum(to_string = "Write N Zeroes to File")]
    WriteNZeroesToFile,
}

/// Makes the border
fn draw_border(r: u8, g: u8, b: u8) {
    if let Some((Width(w), _)) = terminal_size() {
        println!(
            "\n{}\n",
            "─".repeat(w as usize).truecolor(r, g, b).to_string()
        );
    }
}

fn main() {
    loop {
        let result = Select::new("Select function", Functions::VARIANTS.to_vec())
            .with_vim_mode(true)
            .with_page_size(Functions::VARIANTS.len())
            .prompt()
            .unwrap_or_else(|_| exit(0));
        use Functions as F;
        match result {
            F::Collatz => check!(collatz()),
            F::CollatzMaxIter => check!(collatz_max_iter()),
            F::CollatzToN => check!(collatz_to_n()),
            F::DeathClock => check!(run_death_clock().ok()),
            F::Factorial => check!(factorial()),
            F::GuessingGame => check!(guessing_game()),
            F::IsEvenOdd => check!(is_even_odd()),
            F::LinkCleaner => check!(link_cleaner()),
            F::MatrixAverage => check!(matrix_average()),
            F::MatrixTranspose => check!(matrix_transpose()),
            F::MostFrequentLetter => check!(most_frequent_letter()),
            F::NDiceRoll => check!(n_dice_roll()),
            F::NFib => check!(n_fib()),
            F::PercentOff => check!(percent_off()),
            F::StupidEvenOdd => check!(stupid_even_odd()),
            F::SuperFizzBuzz => check!(super_fizz_buzz()),
            F::ThreeDiceRoll => check!(three_n_dice_roll()),
            F::WriteNZeroesToFile => check!(write_n_0s_to_file()),
        }
    }
}

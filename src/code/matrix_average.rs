use inquire::CustomType;
use owo_colors::OwoColorize;
use rand::Rng;
use rayon::prelude::*;

const EDGE: usize = 10;
const MIN: u32 = 1000;
const MAX: u32 = 9999;

pub fn matrix_average() {
    let iter: u32 = CustomType::new("Iterations:")
        .with_help_message("The number of iterations")
        .prompt()
        .unwrap();

    let mut matrix: Vec<Vec<u32>> = vec![vec![0; EDGE]; EDGE];

    matrix.par_iter_mut().for_each(|row| {
        row.iter_mut().for_each(|val| {
            *val = rand::rng().random_range(MIN..=MAX);
        });
    });

    print_matrix(&matrix);

    println!();
    if iter > 1 {
        for _ in 0..iter {
            matrix.par_iter_mut().for_each(|row| {
                row.iter_mut().for_each(|val| {
                    let rand = rand::rng().random_range(MIN..=MAX);
                    *val = (*val + rand) / 2;
                });
            });
        }
        print_matrix(&matrix);
    }
}

fn print_matrix(matrix: &Vec<Vec<u32>>) {
    let (min, max) = min_max(matrix);
    for row in matrix {
        for &val in row {
            let colored_val = gradient_color(val);
            print!("{} ", colored_val);
        }
        println!();
    }
    println!("min: {}", min);
    println!("max: {}", max);
    println!("diff: {}", max - min);
}

fn min_max(matrix: &Vec<Vec<u32>>) -> (u32, u32) {
    matrix
        .par_iter()
        .map(|row| {
            let min = *row.iter().min().unwrap();
            let max = *row.iter().max().unwrap();
            (min, max)
        })
        .reduce(
            || (u32::MAX, u32::MIN),
            |(min1, max1), (min2, max2)| (min1.min(min2), max1.max(max2)),
        )
}

fn gradient_color(val: u32) -> impl std::fmt::Display {
    // Normalize to [0.0, 1.0]
    let t = val as f32 / (MAX - MIN) as f32;

    // Interpolate:
    // green (0,255,0) → yellow (255,255,0) at t=0.5 → red (255,0,0) at t=1
    let (r, g, b) = match t < 0.5 {
        true => {
            // green → yellow
            let f = t / 0.5;
            let r = (255.0 * f) as u8;
            let g = 255;
            let b = 0;
            (r, g, b)
        }
        false => {
            // yellow → red
            let f = (t - 0.5) / 0.5;
            let r = 255;
            let g = (255.0 * (1.0 - f)) as u8;
            let b = 0;
            (r, g, b)
        }
    };

    let s = format!("{:2}", val);
    s.truecolor(r, g, b).to_string()
}

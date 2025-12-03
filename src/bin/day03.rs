use std::io;

use advent_2025::read_input_from_env;

fn silver(input: &str) -> u64 {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let bats = line
            .chars()
            .map(|chr| chr.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>();

        // joltage indices
        let mut upper: usize = 0;
        let mut lower: usize = 1;

        // @TODO: could be computed in loop below
        for j in lower..bats.len() {
            if bats[j] >= bats[lower] {
                lower = j;
            }
        }

        for i in 0..bats.len() - 1 {
            // try to find better upper joltage
            if bats[i] > bats[upper] {
                upper = i;

                // if higher joltage was found, reset lower and repeat
                lower = i + 1;
                for j in lower..bats.len() {
                    if bats[j] >= bats[lower] {
                        lower = j;
                    }
                }
            }
        }

        // println!(
        //     "upper = {} (i: {:3}), lower = {} (j: {:3}), jolt = {}",
        //     bats[upper],
        //     upper,
        //     bats[lower],
        //     lower,
        //     bats[upper] * 10 + bats[lower]
        // );

        sum += bats[upper] as u64 * 10 + bats[lower] as u64;
    }

    sum
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;

    println!("silver: {}", silver(&input));

    Ok(())
}


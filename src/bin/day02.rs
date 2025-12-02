use std::io;

use advent_2025::read_input_from_env;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

fn parse(input: &str) -> Vec<Range> {
    input
        .trim()
        .split(',')
        .filter_map(|range| {
            let (start, end) = range.split_once('-')?;

            let range = Range {
                start: start.parse().ok()?,
                end: end.parse().ok()?,
            };

            Some(range)
        })
        .collect()
}

/// Check if given number has even number of digits
fn even_digits(num: u64) -> bool {
    (num.ilog10() + 1).is_multiple_of(2)
}

/// Splits given (even digit) number into two halves
fn split_num(num: u64) -> (u32, u32) {
    let str = num.to_string(); // welcome to allocation hell
    let (upper, lower) = str.split_at(str.len() / 2);

    (
        u32::from_str_radix(upper, 10).unwrap(),
        u32::from_str_radix(lower, 10).unwrap(),
    )
}

fn silver(input: &[Range]) -> u64 {
    let mut sum = 0;

    for range in input {
        for num in range.start..=range.end {
            // invalid id's will always have even number of digits
            if !even_digits(num) {
                // @TODO: jump to next valid?
                continue;
            }

            let (upper, lower) = split_num(num);
            if upper == lower {
                sum += num;
            }
        }
    }

    sum
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    let input = parse(&input);

    println!("silver: {}", silver(&input));

    Ok(())
}

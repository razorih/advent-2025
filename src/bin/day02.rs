use std::{fmt::Write, io};

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

fn silver(input: &[Range]) -> u64 {
    let mut sum = 0;
    let mut buffer = String::new(); // buffer to hold formatted numbers

    for range in input {
        for num in range.start..=range.end {
            buffer.clear();
            write!(&mut buffer, "{}", num).unwrap();

            // invalid id's will always have even number of digits
            if !buffer.len().is_multiple_of(2) {
                continue;
            }

            let (upper, lower) = buffer.split_at(buffer.len() / 2);

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

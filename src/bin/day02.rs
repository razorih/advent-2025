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

fn gold(input: &[Range]) -> u64 {
    let mut sum = 0;
    let mut buffer = String::new(); // buffer to hold formatted numbers

    for range in input {
        for num in range.start..=range.end {
            buffer.clear();
            write!(&mut buffer, "{}", num).unwrap();

            // iteratively split the string into smaller and smaller chunks
            // all the way into single digit and check if they are all the same
            //
            // for number 1212121212 iterations are
            // 1. -> [12121, 21212] (no match, go on)
            // 2. -> [1212, 1212, 1212] (match! break here)
            for chunk_size in (1..(buffer.len() / 2) + 1).rev() {
                let mut chunker = buffer.as_bytes().chunks_exact(chunk_size);
                let first = chunker.next().unwrap();

                // also after chunker has been exhausted, check if remainder was empty
                // i.e. this number was evenly split
                //
                // if we tried 3093099 with chunk size 3
                // -> [309, 309] (match!), but remainder = [9], so skip
                if chunker.all(|chunk| chunk == first) && chunker.remainder().is_empty() {
                    sum += num;
                    break;
                }
            }
        }
    }

    sum
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    let input = parse(&input);

    println!("silver: {}", silver(&input));
    println!("gold: {}", gold(&input));

    Ok(())
}

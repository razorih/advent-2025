use std::io;

use advent_2025::read_input_from_env;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, num: u64) -> bool {
        self.start <= num && num <= self.end
    }
}

fn parse(input: &str) -> (Vec<Range>, Vec<u64>) {
    let mut lines = input.lines();

    let ranges = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();

            Range {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            }
        })
        .collect();

    // take_while above consumed the newline

    let ids = lines.map(|line| line.parse().unwrap()).collect();

    (ranges, ids)
}

fn silver(ranges: &[Range], ids: &[u64]) -> u64 {
    let mut count = 0;

    'ids: for id in ids {
        for range in ranges {
            if range.contains(*id) {
                count += 1;
                continue 'ids;
            }
        }
    }

    count
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    let (ranges, ids) = parse(&input);

    println!("silver: {}", silver(&ranges, &ids));

    Ok(())
}

use std::io;

use advent_2025::read_input_from_env;

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, num: u64) -> bool {
        self.start <= num && num <= self.end
    }

    /// Number of elements in this range
    fn count(&self) -> u64 {
        self.end - self.start + 1
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

fn gold(ranges: &mut [Range]) -> u64 {
    // first sort ranges so that starts are in ascending order
    ranges.sort_unstable_by_key(|range| range.start);

    let mut sum = 0;
    let mut last = ranges[0];
    for current in &ranges[1..] {
        // make sure that we have the longest range
        // i.e. choose longest:
        //   |-----|
        //   |--------|
        if last.start == current.start {
            last.end = u64::max(last.end, current.end);
            continue;
        }

        // handle merging either
        //   last:  |------|
        //   curr:     |------|
        //    =>    |---------|
        // or
        //   last:  |------|
        //   curr:    |--|
        //    =>    |------|
        if last.end >= current.start {
            last.end = u64::max(last.end, current.end);
            continue;
        }

        // disjoint range, restart
        sum += last.count();
        last = *current;
    }
    sum += last.count(); // leftover last disjoint range

    sum
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    let (mut ranges, ids) = parse(&input);

    println!("silver: {}", silver(&ranges, &ids));
    println!("gold: {}", gold(&mut ranges));

    Ok(())
}

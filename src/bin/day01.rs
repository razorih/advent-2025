use std::io;

use advent_2025::read_input_from_env;

fn silver(input: &str) -> u32 {
    let mut answer = 0;
    let mut dial: i32 = 50;

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let (dir, count) = line.split_at(1);
        let count: i32 = count.parse().unwrap();

        let count = match dir {
            "R" => count,
            "L" => -count,
            _ => count,
        };

        dial = (dial + count).rem_euclid(100);

        if dial == 0 {
            answer += 1;
        }
    }

    answer
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    println!("silver: {}", silver(&input));

    Ok(())
}

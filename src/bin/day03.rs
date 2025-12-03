use std::io;

use advent_2025::read_input_from_env;

fn solve<const N: usize>(banks: &Vec<Vec<u8>>) -> u64 {
    banks.into_iter().map(|bank| joltage::<N>(bank)).sum()
}

/// Calculate joltage of N batteries in given bank
fn joltage<const N: usize>(bank: &[u8]) -> u64 {
    let mut gathered: [u8; N] = [0; N];

    // bank index where we can start looking for next maximum
    let mut cursor = 0;

    for n in 0..N {
        let mut best = 0;

        // can't look till end since then there wouldn't be space for rest of the digits
        for i in cursor..bank.len() - N + n + 1 {
            // >= would give last best, we need first best instead
            if bank[i] > best {
                best = bank[i];
                cursor = i + 1;
            }
        }

        gathered[n] = best;
    }

    gathered
        .into_iter()
        .enumerate()
        .fold(0_u64, |acc, (nth, digit)| {
            let base = 10_u64.pow(N as u32 - nth as u32 - 1);
            acc + (base * digit as u64)
        })
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    let banks: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| chr.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    println!("silver: {}", solve::<2>(&banks));
    println!("gold: {}", solve::<12>(&banks));

    Ok(())
}

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

fn gold(input: &str) -> u32 {
    let mut answer = 0;
    let mut dial: i32 = 50;

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let (dir, count) = line.split_at(1);
        let count = match count.parse::<i32>() {
            Ok(num) => {
                if dir == "L" {
                    -num
                } else {
                    num
                }
            }
            Err(_) => panic!(),
        };

        // from last iteration, "dial" is [0, 99] here
        let next_dial = dial + count;

        if next_dial >= 100 {
            // positive rotation and crossed zero
            answer += (next_dial / 100) as u32; // number of zero crossings
        } else if next_dial <= 0 {
            // negative rotation
            // if dial was previously at 0 we don't want to count that as a zero crossing
            if dial != 0 {
                answer += 1;
            }
            answer += (next_dial.abs() / 100) as u32; // number of full rotations needed to add
        }

        dial = next_dial.rem_euclid(100);
    }

    answer
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    println!("silver: {}", silver(&input));
    println!("gold: {}", gold(&input));

    Ok(())
}

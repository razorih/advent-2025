use std::io;

use advent_2025::{Grid, read_input_from_env};

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Multiply,
}

fn parse(input: &str) -> (Grid<u64>, Vec<Op>) {
    let mut nums = Vec::new();
    let mut ops = Vec::new();

    for thing in input.split_ascii_whitespace() {
        match thing {
            "+" => ops.push(Op::Add),
            "*" => ops.push(Op::Multiply),
            num => nums.push(num.parse().unwrap()),
        }
    }

    let height = nums.len() / ops.len();
    let nums = Grid::from_vec_and_dimensions(nums, ops.len(), height);

    (nums, ops)
}

fn silver(nums: &Grid<u64>, ops: &[Op]) -> u64 {
    let mut accumulator: Vec<u64> = ops
        .iter()
        .map(|op| match op {
            Op::Add => 0,
            Op::Multiply => 1,
        })
        .collect();

    for ((col, _row), num) in nums.iter_indexed() {
        match ops[col] {
            Op::Add => accumulator[col] += num,
            Op::Multiply => accumulator[col] *= num,
        }
    }

    accumulator.into_iter().sum()
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    let (nums, ops) = parse(&input);

    println!("silver: {}", silver(&nums, &ops));

    Ok(())
}

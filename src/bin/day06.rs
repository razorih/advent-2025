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

#[derive(Debug, Clone, Copy)]
enum Thing {
    Digit(u8),
    Space,
}

fn parse_gold(input: &str) -> Grid<Thing> {
    let mut width = 0;
    let mut content = Vec::new();

    // just gather everything into a grid
    for (pos, chr) in input.char_indices() {
        // find width of the grid
        if width == 0 && (chr == '\n' || chr == '\r') {
            width = pos + 1;
        }

        match chr {
            digit if digit.is_ascii_digit() => {
                content.push(Thing::Digit(digit.to_digit(10).unwrap() as u8))
            }
            space if space.is_ascii_whitespace() => content.push(Thing::Space),
            // assumes that first character on the ops line is actually an op (i.e. not a space)
            '+' | '*' => break,
            _ => continue,
        }
    }

    let height = content.len() / width;
    let grid = Grid::from_vec_and_dimensions(content, width, height);

    grid.clone_transposed()
}

/// Get accumulator for given operation type
fn accum_for_op(op: &Op) -> u64 {
    match op {
        Op::Add => 0,
        Op::Multiply => 1,
    }
}

fn silver(nums: &Grid<u64>, ops: &[Op]) -> u64 {
    let mut accumulator: Vec<u64> = ops.iter().map(accum_for_op).collect();

    for ((col, _row), num) in nums.iter_indexed() {
        match ops[col] {
            Op::Add => accumulator[col] += num,
            Op::Multiply => accumulator[col] *= num,
        }
    }

    accumulator.into_iter().sum()
}

fn gold(digits: &Grid<Thing>, ops: &[Op]) -> u64 {
    let mut total = 0;

    let mut problem_i: usize = 0;
    let mut subtotal = accum_for_op(&ops[problem_i]);
    for row_i in 0..digits.height() {
        let mut num: u64 = 0;

        // build the actual number digit by digit
        for digit in digits.iter_row(row_i) {
            if let Thing::Digit(d) = digit {
                num = num * 10 + d as u64;
            }
        }

        // if no digits were gathered, this is an empty line
        // i.e. we moved to another problem
        if num == 0 {
            problem_i += 1;
            total += subtotal;

            // check if we are at last problem
            if let Some(next_op) = ops.get(problem_i) {
                subtotal = accum_for_op(next_op);
            } else {
                break;
            }

            continue;
        }

        match ops[problem_i] {
            Op::Add => subtotal += num,
            Op::Multiply => subtotal *= num,
        }
    }

    total
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    let (nums, ops) = parse(&input);

    println!("silver: {}", silver(&nums, &ops));

    let goldgrid = parse_gold(&input);
    println!("gold: {}", gold(&goldgrid, &ops));

    Ok(())
}

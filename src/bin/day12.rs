use std::io;

use advent_2025::read_input_from_env;

#[derive(Debug)]
struct Problem {
    width: usize,
    height: usize,
    counts: [usize; 6],
}

fn parse(input: &str) -> ([usize; 6], Vec<Problem>) {
    let mut lines = input.lines();

    // scuffed parsing '#'s in each grid
    let mut sizes = [0; 6];
    let mut grid_index = 0;
    let mut total_hashtags = 0;
    for gridline in lines.by_ref().take(30) {
        // assumes that last line is empty
        if gridline.is_empty() {
            sizes[grid_index] = total_hashtags;
            total_hashtags = 0;
            grid_index += 1;
        }

        total_hashtags += gridline.chars().filter(|&chr| chr == '#').count();
    }

    // rest of the lines are problems
    let problems = lines
        .filter_map(|line| {
            let (size, counts) = line.split_once(':')?;

            let (width, height) = size.split_once('x')?;
            let width = width.parse::<usize>().ok()?;
            let height = height.parse::<usize>().ok()?;

            let mut out = Problem {
                width,
                height,
                counts: [0; 6],
            };

            for (i, count) in counts.trim().split_ascii_whitespace().enumerate() {
                out.counts[i] = count.parse().ok()?;
            }

            Some(out)
        })
        .collect();

    (sizes, problems)
}

fn silver(sizes: &[usize], problems: &[Problem]) -> usize {
    // assume that pieces can be perfectly packed
    // i.e. we can completely fill width x height rectangle
    //
    // then each piece contributes a number of '#' into this area,
    // now count total amount of '#'s and compare it to maximal possible area

    let mut total = 0;

    for problem in problems {
        let max_area = problem.width * problem.height;

        let mut filled = 0;
        for (i, count) in problem.counts.iter().enumerate() {
            filled += count * sizes[i];
        }

        if filled <= max_area {
            total += 1;
        }
    }

    total
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    let (sizes, problems) = parse(&input);

    println!("silver: {}", silver(&sizes, &problems));

    Ok(())
}

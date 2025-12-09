use std::io;

use advent_2025::read_input_from_env;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn area_with(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

fn silver(points: &[Point]) -> u64 {
    let mut max = 0;

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let area = points[i].area_with(&points[j]);

            if area > max {
                max = area;
            }
        }
    }

    max
}

fn parse(input: &str) -> Vec<Point> {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let (x, y) = line.split_once(',')?;
            let x = x.parse().ok()?;
            let y = y.parse().ok()?;

            Some(Point { x, y })
        })
        .collect()
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    let points = parse(&input);

    println!("silver: {}", silver(&points));

    Ok(())
}

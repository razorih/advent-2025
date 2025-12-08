use std::{collections::HashMap, io};

use advent_2025::read_input_from_env;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    fn euclidean_distance(&self, other: &Self) -> f64 {
        let dx = self.x.abs_diff(other.x) as f64;
        let dy = self.y.abs_diff(other.y) as f64;
        let dz = self.z.abs_diff(other.z) as f64;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn brute_force_sorted_distances(points: &[Point]) -> Vec<(usize, usize, f64)> {
    let mut out = Vec::with_capacity(points.len() * points.len());

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let dist = points[i].euclidean_distance(&points[j]);

            out.push((i, j, dist));
        }
    }

    out.sort_unstable_by(|(_, _, d1), (_, _, d2)| d1.total_cmp(d2));

    out
}

fn silver(points: &[Point]) -> u64 {
    let distances = brute_force_sorted_distances(points);

    // mapping of point index => circuit id
    let mut circuits: HashMap<usize, usize> = HashMap::with_capacity(points.len());

    for (i, j, _dist) in distances.into_iter().take(1000) {
        match (circuits.get(&i), circuits.get(&j)) {
            (None, None) => {
                // brand new circuit "i"
                circuits.insert(i, i);
                circuits.insert(j, i);
            }
            (None, Some(&circ)) => {
                // merge i to j circuit
                circuits.insert(i, circ);
            }
            (Some(&circ), None) => {
                // merge j to i circuit
                circuits.insert(j, circ);
            }
            (Some(&c1), Some(&c2)) => {
                // need to merge two circuits together
                // just put c2 => c1
                circuits.values_mut().for_each(|circ| {
                    if circ == &c2 {
                        *circ = c1;
                    }
                });
            }
        }
    }

    // stupid circuit size counting
    let mut counts: Vec<usize> = circuits
        .into_values()
        .fold(HashMap::<usize, usize>::new(), |mut acc, circuit| {
            *acc.entry(circuit).or_default() += 1;
            acc
        })
        .into_values()
        .collect();

    counts.sort_unstable();
    counts.into_iter().rev().take(3).product::<usize>() as u64
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    let points = parse(&input);

    println!("silver: {}", silver(&points));

    Ok(())
}

fn parse(input: &str) -> Vec<Point> {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(',');

            let x = parts.next()?.parse().ok()?;
            let y = parts.next()?.parse().ok()?;
            let z = parts.next()?.parse().ok()?;

            Some(Point { x, y, z })
        })
        .collect()
}

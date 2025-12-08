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
        // abs_diff is fine since we're squaring anyway
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

fn solve<const GOLD: bool>(points: &[Point]) -> u64 {
    let distances = brute_force_sorted_distances(points);

    // mapping of point index => circuit id
    // by default each point creates its own circuit
    let mut circuits: HashMap<usize, usize> = (0..points.len()).map(|i| (i, i)).collect();

    let distance_slice = if GOLD {
        &distances[..]
    } else {
        &distances[..1000]
    };

    for &(i, j, _dist) in distance_slice {
        match (circuits.get(&i), circuits.get(&j)) {
            (Some(&c1), Some(&c2)) if c1 != c2 => {
                // need to merge two circuits together
                // just put c2 => c1
                circuits.values_mut().for_each(|circ| {
                    if circ == &c2 {
                        *circ = c1;
                    }
                });
            }
            // c1 == c2 intra-network merge, skip
            _ => continue,
        }

        if GOLD {
            // slightly less stupid circuit size counting (now in a loop)
            let circuit_count = circuits
                .values()
                .fold(HashMap::<usize, usize>::new(), |mut acc, circuit| {
                    *acc.entry(*circuit).or_insert(0) += 1;
                    acc
                })
                .len();

            if circuit_count == 1 {
                // final merge was just completed,
                // since c1 and c2 still hold circuit ids that were just merged
                // fetch original points using i and j
                return points[i].x * points[j].x;
            }
        }
    }

    // silver only below here

    // stupid circuit size counting
    let mut counts: Vec<usize> = circuits
        .values()
        .fold(HashMap::<usize, usize>::new(), |mut acc, circuit| {
            *acc.entry(*circuit).or_insert(0) += 1;
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

    println!("silver: {}", solve::<false>(&points));
    println!("gold: {}", solve::<true>(&points));

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

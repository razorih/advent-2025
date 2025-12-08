use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Write},
    io,
};

use advent_2025::{Grid, read_input_from_env};

#[derive(Debug, Clone, Copy)]
enum Tile {
    Start,
    Empty,
    Splitter,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Start => f.write_char('S'),
            Tile::Empty => f.write_char('.'),
            Tile::Splitter => f.write_char('^'),
        }
    }
}

fn silver(grid: &Grid<Tile>) -> u64 {
    let (start_col, _) = grid
        .find_one_pos_by(|tile| matches!(tile, Tile::Start))
        .unwrap();

    // set of columns which currently have an active beam
    let mut beam_cols: HashSet<usize> = HashSet::with_capacity(grid.width());
    beam_cols.insert(start_col);

    let mut splits = 0;
    for ((col, _row), tile) in grid.iter_indexed() {
        match tile {
            Tile::Splitter => {
                if beam_cols.contains(&col) {
                    splits += 1;
                    beam_cols.remove(&col);

                    beam_cols.insert(&col - 1);
                    beam_cols.insert(&col + 1);
                }
            }
            _ => continue,
        }
    }

    splits
}

/// Look for a splitter below some coordinate.
/// Returns the same coordinate if the coordinate itself contains a splitter
fn look_down_for_splitter(
    grid: &Grid<Tile>,
    (start_col, start_row): (usize, usize),
) -> Option<(usize, usize)> {
    for row in start_row..grid.height() {
        if let Some(Tile::Splitter) = grid.at(start_col, row) {
            return Some((start_col, row));
        }
    }

    None
}

fn path_count(
    grid: &Grid<Tile>,
    memo: &mut HashMap<(usize, usize), u64>,
    (col, row): (usize, usize),
) -> u64 {
    if let Some(&remembered) = memo.get(&(col, row)) {
        return remembered;
    }

    let left_paths = if let Some(next) = look_down_for_splitter(grid, (col - 1, row)) {
        path_count(grid, memo, next)
    } else {
        1 // base case, beam goes out of bounds
    };

    let right_paths = if let Some(next) = look_down_for_splitter(grid, (col + 1, row)) {
        path_count(grid, memo, next)
    } else {
        1 // base case, beam goes out of bounds
    };

    // base case
    memo.insert((col, row), left_paths + right_paths);
    left_paths + right_paths
}

fn gold(grid: &Grid<Tile>) -> u64 {
    let (start_col, start_row) = grid
        .find_one_pos_by(|tile| matches!(tile, Tile::Start))
        .expect("failed to find start position");

    let mut visited: HashMap<(usize, usize), u64> = HashMap::new();

    path_count(
        grid,
        &mut visited,
        look_down_for_splitter(grid, (start_col, start_row)).unwrap(),
    )
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    let grid = Grid::new(&input, |chr, _| match chr {
        'S' => Tile::Start,
        '.' => Tile::Empty,
        '^' => Tile::Splitter,
        _ => panic!("invalid tile"),
    });

    println!("silver: {}", silver(&grid));
    println!("gold: {}", gold(&grid));

    Ok(())
}

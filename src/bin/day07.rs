use std::{
    collections::HashSet,
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

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    let grid = Grid::new(&input, |chr, _| match chr {
        'S' => Tile::Start,
        '.' => Tile::Empty,
        '^' => Tile::Splitter,
        _ => panic!("invalid tile"),
    });

    println!("silver: {}", silver(&grid));

    Ok(())
}

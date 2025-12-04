use std::io;

use advent_2025::{Grid, read_input_from_env};

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Roll,
}

fn check_tile(grid: &Grid<Tile>, col: usize, row: usize) -> bool {
    let checked_directions: [(isize, isize); 8] = [
        (0, -1),
        (0, 1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, -1),
        (-1, 1),
    ];

    let entry = grid.entry(col, row);
    let n_adjacent_rolls = checked_directions
        .into_iter()
        .filter(|(dx, dy)| matches!(entry.at_offset(*dx, *dy), Some(Tile::Roll)))
        .count();

    n_adjacent_rolls < 4
}

fn silver(grid: &Grid<Tile>) -> usize {
    grid.iter_indexed()
        .filter(|((col, row), tile)| matches!(tile, Tile::Roll) && check_tile(grid, *col, *row))
        .count()
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    let grid = Grid::new(&input, |chr, _| match chr {
        '.' => Tile::Empty,
        '@' => Tile::Roll,
        _ => panic!("invalid tile in input"),
    });

    println!("silver: {}", silver(&grid));

    Ok(())
}

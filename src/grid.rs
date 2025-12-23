#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    content: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy> Grid<T> {
    pub fn new(content: &str, transform: impl Fn(char, (usize, usize)) -> T) -> Self {
        // count number of lines (ignoring empty lines) and length with of first line
        let height = content.lines().filter(|line| !line.is_empty()).count();
        let width = content.lines().next().unwrap().len();

        let content: Vec<T> = content
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .enumerate()
            .map(|(i, c)| transform(c, (i % width, i / width)))
            .collect();

        // make sure we have an actual grid
        debug_assert_eq!(content.len(), height * width);

        Self {
            content,
            width,
            height,
        }
    }

    pub fn from_vec_and_dimensions(content: Vec<T>, width: usize, height: usize) -> Self {
        if content.len() != width * height {
            panic!("invalid grid dimensions");
        }

        Self {
            content,
            width,
            height,
        }
    }

    fn content(&self) -> &[T] {
        self.content.as_slice()
    }

    fn content_mut(&mut self) -> &mut [T] {
        self.content.as_mut_slice()
    }

    pub fn at(&self, col: usize, row: usize) -> Option<T> {
        let index = col + row * self.width;
        self.content.get(index).copied()
    }

    pub fn at_mut(&mut self, col: usize, row: usize) -> Option<&mut T> {
        let index = col + row * self.width;
        self.content.get_mut(index)
    }

    pub fn entry(&self, col: usize, row: usize) -> GridEntry<'_, T> {
        GridEntry {
            grid: self,
            col,
            row,
        }
    }

    pub fn entry_mut(&mut self, col: usize, row: usize) -> GridEntryMut<'_, T> {
        GridEntryMut {
            grid: self,
            col,
            row,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn iter_indexed(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        let width = self.width();
        self.content.iter().enumerate().map(move |(i, c)| {
            let pos = (i % width, i / width);
            (pos, c)
        })
    }

    /// Find position of one item by some predicate.
    ///
    /// Useful for finding starting positions etc..
    pub fn find_one_pos_by(&self, pred: impl Fn(T) -> bool) -> Option<(usize, usize)> {
        for (pos, &t) in self.iter_indexed() {
            if pred(t) {
                return Some(pos);
            }
        }
        None
    }

    /// Get an iterator over given row
    ///
    /// Returns an empty iterator if row is out of bounds
    pub fn iter_row(&self, row: usize) -> impl Iterator<Item = T> {
        self.content
            .iter()
            .skip(row * self.width)
            .take(self.width)
            .copied()
    }

    /// Get an iterator over given column
    ///
    /// Returns an empty iterator if column is out of bounds
    pub fn iter_col(&self, col: usize) -> impl Iterator<Item = T> {
        self.content.iter().skip(col).step_by(self.width).copied()
    }

    /// Transposes itself in-place if the grid is square
    ///
    /// Panics if the grid is not square
    ///
    /// Pseudocode from:
    /// https://en.wikipedia.org/wiki/In-place_matrix_transposition#Square_matrices
    pub fn transpose_square(&mut self) {
        if self.width != self.height {
            panic!("grid is not square");
        }

        #[allow(non_snake_case)]
        let N = self.width;

        for row in 0..N - 1 {
            for col in row + 1..N {
                let src = row * N + col;
                let dst = col * N + row;

                self.content.swap(src, dst);
            }
        }
    }

    /// Creates a transposed copy of the grid
    ///
    /// If your grid is a square,
    /// you may want to use [`Self::transpose_square`] instead
    pub fn clone_transposed(&self) -> Self {
        // all elements will be overwritten
        // so this is just an allocation
        // without needing T: Default or MaybeUninit
        let mut new_content = self.content.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let src = row * self.width + col;
                let dst = col * self.height + row;

                new_content[dst] = self.content[src];
            }
        }

        Grid {
            content: new_content,
            width: self.height,
            height: self.width,
        }
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let index = col + row * self.width;
                write!(f, "{}", self.content[index])?;
            }

            if row + 1 != self.height {
                // print newline if this isn't the last row
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

/// Helper for working with offsets
#[derive(Debug)]
pub struct GridEntry<'a, T> {
    grid: &'a Grid<T>,
    col: usize,
    row: usize,
}

// todo: get rid of Copy bound and make callers rely on .copied() ?
impl<'a, T: Copy> GridEntry<'a, T> {
    pub fn at_offset(&self, col_offset: isize, row_offset: isize) -> Option<T> {
        self.offset(col_offset, row_offset).map(|thing| thing.0)
    }

    /// Returns item at offset and its true column and row index if valid
    pub fn offset(&self, col_offset: isize, row_offset: isize) -> Option<(T, usize, usize)> {
        let Some(true_col) = self.col.checked_add_signed(col_offset) else {
            return None;
        };
        let Some(true_row) = self.row.checked_add_signed(row_offset) else {
            return None;
        };

        if true_col >= self.grid.width() {
            return None;
        }
        if true_row >= self.grid.height() {
            return None;
        }

        if let Some(thing) = self.grid.at(true_col, true_row) {
            Some((thing, true_col, true_row))
        } else {
            None
        }
    }
}

// allow converting mutable grid entry into immutable one
impl<'a, T> From<GridEntryMut<'a, T>> for GridEntry<'a, T> {
    fn from(value: GridEntryMut<'a, T>) -> Self {
        GridEntry {
            grid: value.grid,
            col: value.col,
            row: value.row,
        }
    }
}

pub struct GridEntryMut<'a, T> {
    grid: &'a mut Grid<T>,
    col: usize,
    row: usize,
}

impl<T: Copy> GridEntryMut<'_, T> {
    pub fn at_offset_mut(&mut self, col_offset: isize, row_offset: isize) -> Option<&mut T> {
        self.offset_mut(col_offset, row_offset).map(|thing| thing.0)
    }

    /// Returns item at offset and its true column and row index if valid
    pub fn offset_mut(
        &mut self,
        col_offset: isize,
        row_offset: isize,
    ) -> Option<(&mut T, usize, usize)> {
        let Some(true_col) = self.col.checked_add_signed(col_offset) else {
            return None;
        };
        let Some(true_row) = self.row.checked_add_signed(row_offset) else {
            return None;
        };

        if true_col >= self.grid.width() {
            return None;
        }
        if true_row >= self.grid.height() {
            return None;
        }

        if let Some(thing) = self.grid.at_mut(true_col, true_row) {
            Some((thing, true_col, true_row))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_general() {
        let grid = Grid::from_vec_and_dimensions(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let transposed = grid.clone_transposed();

        assert_eq!(transposed.content(), &[1, 3, 5, 2, 4, 6]);
    }

    #[test]
    fn transpose_square() {
        let mut grid = Grid::from_vec_and_dimensions(vec![1, 2, 3, 4], 2, 2);

        grid.transpose_square();

        assert_eq!(grid.content(), &[1, 3, 2, 4]);
    }

    #[test]
    fn transpose_vec() {
        let grid = Grid::from_vec_and_dimensions(vec![1, 2, 3, 4, 5], 1, 5);
        let transposed = grid.clone_transposed();

        assert_eq!(transposed.content(), &[1, 2, 3, 4, 5]);
        assert_eq!(transposed.width(), 5);
        assert_eq!(transposed.height(), 1);
    }

    #[test]
    fn iterates_row() {
        let grid = Grid::from_vec_and_dimensions(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);
        assert!(grid.iter_row(1).eq([4, 5, 6]));
    }

    #[test]
    fn iterates_col() {
        let grid = Grid::from_vec_and_dimensions(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);
        assert!(grid.iter_col(1).eq([2, 5, 8]));
    }

    #[test]
    fn out_of_bounds_iterators_are_empty() {
        let grid = Grid::from_vec_and_dimensions(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);

        assert_eq!(grid.iter_row(69).next(), None);
        assert_eq!(grid.iter_col(420).next(), None);
    }
}

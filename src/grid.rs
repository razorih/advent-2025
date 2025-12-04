#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub content: Vec<T>,
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

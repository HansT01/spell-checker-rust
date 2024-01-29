use std::ops::Index;

#[derive(Debug)]
pub struct Vec2D<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Default + Clone> Vec2D<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        let size: usize = rows * cols;
        Self {
            data: vec![T::default(); size],
            rows,
            cols,
        }
    }

    pub fn from_slice(rows: usize, cols: usize, slice: &[T]) -> Self {
        Self {
            data: slice.into(),
            rows,
            cols,
        }
    }
}

impl<T> Index<usize> for Vec2D<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.cols;
        let end = start + self.cols;
        &self.data[start..end]
    }
}

impl<T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.cols + col]
    }
}

use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Vec2D<T> {
    data: Vec<T>,
    _rows: usize,
    _cols: usize,
}

impl<T: Default + Clone> Vec2D<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        let size: usize = rows * cols;
        Self {
            data: vec![T::default(); size],
            _rows: rows,
            _cols: cols,
        }
    }

    pub fn from_slice(rows: usize, cols: usize, slice: &[T]) -> Self {
        let expected_length = rows * cols;
        assert_eq!(
            expected_length,
            slice.len(),
            "Invalid slice length for Vec2D creation. Expected length: {}, Actual length: {}",
            expected_length,
            slice.len()
        );
        Self {
            data: slice.into(),
            _rows: rows,
            _cols: cols,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks(self._cols)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.data.chunks_mut(self._cols)
    }
}

impl<T> Index<usize> for Vec2D<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self._cols;
        let end = start + self._cols;
        &self.data[start..end]
    }
}

impl<T> IndexMut<usize> for Vec2D<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * self._cols;
        let end = start + self._cols;
        &mut self.data[start..end]
    }
}

use std::{
    alloc::{alloc, Layout},
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct Array2D<T> {
    data: Box<[T]>,
    _rows: usize,
    _cols: usize,
}

impl<T> Array2D<T> {
    pub fn new(rows: usize, cols: usize) -> Self
    where
        T: Default,
    {
        let size: usize = rows * cols;
        let layout = Layout::array::<T>(size).unwrap();
        let ptr = unsafe { alloc(layout) as *mut T };
        for i in 0..size {
            unsafe {
                ptr.add(i).write(Default::default());
            }
        }
        Self {
            data: unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, size)) },
            _rows: rows,
            _cols: cols,
        }
    }

    pub fn from_slice(rows: usize, cols: usize, slice: &[T]) -> Self
    where
        T: Copy,
    {
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

impl<T> Index<usize> for Array2D<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self._cols;
        let end = start + self._cols;
        &self.data[start..end]
    }
}

impl<T> IndexMut<usize> for Array2D<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * self._cols;
        let end = start + self._cols;
        &mut self.data[start..end]
    }
}

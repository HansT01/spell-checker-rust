use std::{
    alloc::{alloc, Layout},
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct Array2D<T> {
    data: Box<[T]>,
    _num_rows: usize,
    _num_cols: usize,
}

impl<T> Array2D<T> {
    pub fn new(num_rows: usize, num_cols: usize) -> Self
    where
        T: Default,
    {
        let size: usize = num_rows * num_cols;
        let layout = Layout::array::<T>(size).unwrap();
        let ptr = unsafe { alloc(layout) as *mut T };
        for i in 0..size {
            unsafe {
                ptr.add(i).write(Default::default());
            }
        }
        Self {
            data: unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, size)) },
            _num_rows: num_rows,
            _num_cols: num_cols,
        }
    }

    pub fn new_with_default(num_rows: usize, num_cols: usize, default_value: T) -> Self
    where
        T: Copy,
    {
        let size: usize = num_rows * num_cols;
        let layout = Layout::array::<T>(size).unwrap();
        let ptr = unsafe { alloc(layout) as *mut T };
        for i in 0..size {
            unsafe {
                ptr.add(i).write(default_value);
            }
        }
        Self {
            data: unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, size)) },
            _num_rows: num_rows,
            _num_cols: num_cols,
        }
    }

    pub fn from_slice(num_rows: usize, num_cols: usize, slice: &[T]) -> Self
    where
        T: Copy,
    {
        let expected_length = num_rows * num_cols;
        assert_eq!(
            expected_length,
            slice.len(),
            "Invalid slice length for Array2D creation. Expected length: {}, Actual length: {}",
            expected_length,
            slice.len()
        );
        Self {
            data: slice.into(),
            _num_rows: num_rows,
            _num_cols: num_cols,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks(self._num_cols)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.data.chunks_mut(self._num_cols)
    }
}

impl<T> Index<usize> for Array2D<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self._num_cols;
        let end = start + self._num_cols;
        &self.data[start..end]
    }
}

impl<T> IndexMut<usize> for Array2D<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * self._num_cols;
        let end = start + self._num_cols;
        &mut self.data[start..end]
    }
}

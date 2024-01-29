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

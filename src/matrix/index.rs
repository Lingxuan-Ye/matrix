use super::order::Order;
use super::shape::AxisShape;
use super::Matrix;
use crate::error::{Error, Result};

/// A structure for indexing a [`Matrix`]. You might prefer using
/// `(usize, usize)` instead. Refer to [`IndexLike`] for more information.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Index {
    pub row: usize,
    pub col: usize,
}

impl Index {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

/// Any type implementing this trait can be used to index a [`Matrix`].
///
/// # Examples
///
/// ```
/// use matreex::{Index, matrix};
///
/// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
///
/// assert_eq!(matrix[Index::new(0, 0)], 0);
/// assert_eq!(matrix[(0, 0)], 0);
/// assert_eq!(matrix[[0, 0]], 0);
/// ```
pub trait IndexLike {
    fn row(&self) -> usize;

    fn col(&self) -> usize;

    fn is_out_of_bounds_of<T>(&self, matrix: &Matrix<T>) -> bool {
        let shape = matrix.shape();
        self.row() >= shape.nrows || self.col() >= shape.ncols
    }
}

impl IndexLike for Index {
    fn row(&self) -> usize {
        self.row
    }

    fn col(&self) -> usize {
        self.col
    }
}

impl IndexLike for (usize, usize) {
    fn row(&self) -> usize {
        self.0
    }

    fn col(&self) -> usize {
        self.1
    }
}

impl IndexLike for [usize; 2] {
    fn row(&self) -> usize {
        self[0]
    }

    fn col(&self) -> usize {
        self[1]
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct AxisIndex {
    major: usize,
    minor: usize,
}

impl AxisIndex {
    fn new<I: IndexLike>(index: I, order: Order) -> Self {
        let (major, minor) = match order {
            Order::RowMajor => (index.row(), index.col()),
            Order::ColMajor => (index.col(), index.row()),
        };
        Self { major, minor }
    }

    fn transpose(&mut self) -> &mut Self {
        (self.major, self.minor) = (self.minor, self.major);
        self
    }

    fn from_flattened_unchecked(index: usize, shape: AxisShape) -> Self {
        let major = index / shape.major_stride();
        // let minor = (index % shape.major_stride()) / shape.minor_stride();
        let minor = index % shape.major_stride();
        Self { major, minor }
    }

    fn into_flattened_unchecked(self, shape: AxisShape) -> usize {
        // self.major * shape.major_stride() + self.minor * shape.minor_stride()
        self.major * shape.major_stride() + self.minor
    }
}

impl<T> Matrix<T> {
    pub(super) fn flatten_index_unchecked<I: IndexLike>(&self, index: I) -> usize {
        AxisIndex::new(index, self.order).into_flattened_unchecked(self.shape)
    }

    pub(super) fn try_flatten_index<I: IndexLike>(&self, index: I) -> Result<usize> {
        let index = AxisIndex::new(index, self.order);
        if index.major >= self.major() || index.minor >= self.minor() {
            return Err(Error::IndexOutOfBounds);
        }
        Ok(index.into_flattened_unchecked(self.shape))
    }

    pub(super) fn flatten_index<I: IndexLike>(&self, index: I) -> usize {
        match self.try_flatten_index(index) {
            Err(error) => panic!("{error}"),
            Ok(index) => index,
        }
    }
}

impl<T> Matrix<T> {
    /// Returns a reference to an element at given position,
    /// or [`Error::IndexOutOfBounds`] if out of bounds.
    pub fn get<I: IndexLike>(&self, index: I) -> Result<&T> {
        let index = self.try_flatten_index(index)?;
        unsafe { Ok(self.data.get_unchecked(index)) }
    }

    /// Returns a mutable reference to an element at given position,
    /// or [`Error::IndexOutOfBounds`] if out of bounds.
    pub fn get_mut<I: IndexLike>(&mut self, index: I) -> Result<&mut T> {
        let index = self.try_flatten_index(index)?;
        unsafe { Ok(self.data.get_unchecked_mut(index)) }
    }

    /// Returns a reference to an element without doing bounds checking.
    ///
    /// For a safe alternative see [`get`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [`get`]: Matrix::get
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn get_unchecked<I: IndexLike>(&self, index: I) -> &T {
        let index = self.flatten_index_unchecked(index);
        unsafe { self.data.get_unchecked(index) }
    }

    /// Returns a mutable reference to an element without doing bounds checking.
    ///
    /// For a safe alternative see [`get_mut`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior
    /// even if the resulting reference is not used.
    ///
    /// [`get_mut`]: Matrix::get_mut
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn get_unchecked_mut<I: IndexLike>(&mut self, index: I) -> &mut T {
        let index = self.flatten_index_unchecked(index);
        unsafe { self.data.get_unchecked_mut(index) }
    }
}

impl<T, I> std::ops::Index<I> for Matrix<T>
where
    I: IndexLike,
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        let index = self.flatten_index(index);
        &self.data[index]
    }
}

impl<T, I> std::ops::IndexMut<I> for Matrix<T>
where
    I: IndexLike,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let index = self.flatten_index(index);
        &mut self.data[index]
    }
}

pub(super) fn translate_index_between_orders_unchecked(
    index: usize,
    src_shape: AxisShape,
) -> usize {
    let mut index = AxisIndex::from_flattened_unchecked(index, src_shape);
    index.transpose();
    let mut dest_shape = src_shape;
    dest_shape.transpose();
    index.into_flattened_unchecked(dest_shape)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::matrix;

    #[test]
    fn test_index_new() {
        let target = Index { row: 2, col: 3 };

        assert_eq!(Index::new(2, 3), target);
        assert_ne!(Index::new(3, 2), target);
    }

    #[test]
    fn test_index_display() {
        assert_eq!(Index::new(2, 3).to_string(), "(2, 3)");
        assert_eq!(Index::new(3, 2).to_string(), "(3, 2)");
    }

    #[test]
    fn test_index_like() {
        let matrix = matrix![[0, 1, 2], [3, 4, 5]];

        assert_eq!(Index::new(2, 3).row(), 2);
        assert_eq!(Index::new(2, 3).col(), 3);
        assert!(!Index::new(1, 2).is_out_of_bounds_of(&matrix));
        assert!(Index::new(1, 3).is_out_of_bounds_of(&matrix));
        assert!(Index::new(2, 2).is_out_of_bounds_of(&matrix));
        assert!(Index::new(2, 3).is_out_of_bounds_of(&matrix));

        assert_eq!((2, 3).row(), 2);
        assert_eq!((2, 3).col(), 3);
        assert!(!(1, 2).is_out_of_bounds_of(&matrix));
        assert!((1, 3).is_out_of_bounds_of(&matrix));
        assert!((2, 2).is_out_of_bounds_of(&matrix));
        assert!((2, 3).is_out_of_bounds_of(&matrix));

        assert_eq!([2, 3].row(), 2);
        assert_eq!([2, 3].col(), 3);
        assert!(![1, 2].is_out_of_bounds_of(&matrix));
        assert!([1, 3].is_out_of_bounds_of(&matrix));
        assert!([2, 2].is_out_of_bounds_of(&matrix));
        assert!([2, 3].is_out_of_bounds_of(&matrix));
    }

    #[test]
    fn test_matrix_get() {
        let matrix = matrix![[0, 1, 2], [3, 4, 5]];
        assert_eq!(matrix.get((0, 0)), Ok(&0));
        assert_eq!(matrix.get((0, 1)), Ok(&1));
        assert_eq!(matrix.get((0, 2)), Ok(&2));
        assert_eq!(matrix.get((1, 0)), Ok(&3));
        assert_eq!(matrix.get((1, 1)), Ok(&4));
        assert_eq!(matrix.get((1, 2)), Ok(&5));
        assert_eq!(matrix.get((2, 0)), Err(Error::IndexOutOfBounds));
    }

    #[test]
    fn test_matrix_get_mut() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
        assert_eq!(matrix.get_mut((0, 0)), Ok(&mut 0));
        assert_eq!(matrix.get_mut((0, 1)), Ok(&mut 1));
        assert_eq!(matrix.get_mut((0, 2)), Ok(&mut 2));
        assert_eq!(matrix.get_mut((1, 0)), Ok(&mut 3));
        assert_eq!(matrix.get_mut((1, 1)), Ok(&mut 4));
        assert_eq!(matrix.get_mut((1, 2)), Ok(&mut 5));
        assert_eq!(matrix.get_mut((2, 0)), Err(Error::IndexOutOfBounds));
    }

    #[test]
    fn test_matrix_get_unchecked() {
        let matrix = matrix![[0, 1, 2], [3, 4, 5]];
        unsafe {
            assert_eq!(matrix.get_unchecked((0, 0)), &0);
            assert_eq!(matrix.get_unchecked((0, 1)), &1);
            assert_eq!(matrix.get_unchecked((0, 2)), &2);
            assert_eq!(matrix.get_unchecked((1, 0)), &3);
            assert_eq!(matrix.get_unchecked((1, 1)), &4);
            assert_eq!(matrix.get_unchecked((1, 2)), &5);
        }
    }

    #[test]
    fn test_matrix_get_unchecked_mut() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
        unsafe {
            assert_eq!(matrix.get_unchecked_mut((0, 0)), &mut 0);
            assert_eq!(matrix.get_unchecked_mut((0, 1)), &mut 1);
            assert_eq!(matrix.get_unchecked_mut((0, 2)), &mut 2);
            assert_eq!(matrix.get_unchecked_mut((1, 0)), &mut 3);
            assert_eq!(matrix.get_unchecked_mut((1, 1)), &mut 4);
            assert_eq!(matrix.get_unchecked_mut((1, 2)), &mut 5);
        }
    }

    #[test]
    fn test_matrix_index() {
        let matrix = matrix![[0, 1, 2], [3, 4, 5]];
        assert_eq!(matrix[(0, 0)], 0);
        assert_eq!(matrix[(0, 1)], 1);
        assert_eq!(matrix[(0, 2)], 2);
        assert_eq!(matrix[(1, 0)], 3);
        assert_eq!(matrix[(1, 1)], 4);
        assert_eq!(matrix[(1, 2)], 5);
    }

    #[test]
    fn test_matrix_index_mut() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
        matrix[(0, 0)] += 1;
        matrix[(0, 1)] += 1;
        matrix[(0, 2)] += 1;
        matrix[(1, 0)] += 1;
        matrix[(1, 1)] += 1;
        matrix[(1, 2)] += 1;
        assert_eq!(matrix, matrix![[1, 2, 3], [4, 5, 6]])
    }

    #[test]
    #[should_panic]
    fn test_row_out_of_bounds() {
        let matrix = matrix![[0, 1, 2], [3, 4, 5]];
        let _ = matrix[(2, 0)];
    }

    #[test]
    #[should_panic]
    fn test_col_out_of_bounds() {
        let matrix = matrix![[0, 1, 2], [3, 4, 5]];
        let _ = matrix[(0, 3)];
    }
}

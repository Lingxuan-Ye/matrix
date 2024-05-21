use super::order::Order;
use super::shape::AxisShape;
use super::Matrix;
use crate::error::{Error, Result};

/// A helper trait used for [`Matrix`] indexing.
///
/// # Safety
///
/// Marking this trait as `unsafe` originates from a poor imitation
/// of [`SliceIndex`]. In another words, I have no idea what I'm doing.
///
/// [`SliceIndex`]: core::slice::SliceIndex
pub unsafe trait MatrixIndex<T>: internal::Sealed {
    /// The output type returned by methods.
    type Output;

    /// Returns a reference to the output at this location,
    /// if in bounds.
    fn get(self, matrix: &Matrix<T>) -> Result<&Self::Output>;

    /// Returns a mutable reference to the output at this location,
    /// if in bounds.
    fn get_mut(self, matrix: &mut Matrix<T>) -> Result<&mut Self::Output>;

    /// Returns a reference to the output at this location without
    /// doing bounds checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*.
    ///
    /// For a safe alternative see [`get`].
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    /// [`get`]: MatrixIndex::get
    unsafe fn get_unchecked(self, matrix: &Matrix<T>) -> &Self::Output;

    /// Returns a mutable reference to the output at this location without
    /// doing bounds checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*.
    ///
    /// For a safe alternative see [`get_mut`].
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    /// [`get_mut`]: MatrixIndex::get_mut
    unsafe fn get_unchecked_mut(self, matrix: &mut Matrix<T>) -> &mut Self::Output;

    /// Returns a reference to the output at this location.
    ///
    /// # Panics
    ///
    /// Panics if out of bounds.
    fn index(self, matrix: &Matrix<T>) -> &Self::Output;

    /// Returns a mutable reference to the output at this location.
    ///
    /// # Panics
    ///
    /// Panics if out of bounds.
    fn index_mut(self, matrix: &mut Matrix<T>) -> &mut Self::Output;
}

impl<T> Matrix<T> {
    /// Returns a reference to the [`MatrixIndex::Output`]
    /// at given location.
    ///
    /// # Errors
    ///
    /// - [`Error::IndexOutOfBounds`] if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{matrix, Error};
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    /// assert_eq!(matrix.get((1, 1)), Ok(&4));
    /// assert_eq!(matrix.get((2, 3)), Err(Error::IndexOutOfBounds));
    /// ```
    pub fn get<I>(&self, index: I) -> Result<&I::Output>
    where
        I: MatrixIndex<T>,
    {
        index.get(self)
    }

    /// Returns a mutable reference to the [`MatrixIndex::Output`]
    /// at given location.
    ///
    /// # Errors
    ///
    /// - [`Error::IndexOutOfBounds`] if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{matrix, Error};
    ///
    /// let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
    /// assert_eq!(matrix.get_mut((1, 1)), Ok(&mut 4));
    /// assert_eq!(matrix.get_mut((2, 3)), Err(Error::IndexOutOfBounds));
    /// ```
    pub fn get_mut<I>(&mut self, index: I) -> Result<&mut I::Output>
    where
        I: MatrixIndex<T>,
    {
        index.get_mut(self)
    }

    /// Returns a reference to the [`MatrixIndex::Output`]
    /// at given location without doing bounds checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*.
    ///
    /// For a safe alternative see [`get`].
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{matrix, Error};
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    /// unsafe { assert_eq!(matrix.get_unchecked((1, 1)), &4); }
    /// ```
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    /// [`get`]: Matrix::get
    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: MatrixIndex<T>,
    {
        unsafe { index.get_unchecked(self) }
    }

    /// Returns a mutable reference to the [`MatrixIndex::Output`]
    /// at given location without doing bounds checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*.
    ///
    /// For a safe alternative see [`get_mut`].
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{matrix, Error};
    ///
    /// let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
    /// unsafe { assert_eq!(matrix.get_unchecked_mut((1, 1)), &mut 4); }
    /// ```
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    /// [`get_mut`]: Matrix::get_mut
    pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut I::Output
    where
        I: MatrixIndex<T>,
    {
        unsafe { index.get_unchecked_mut(self) }
    }
}

impl<T, I> std::ops::Index<I> for Matrix<T>
where
    I: MatrixIndex<T>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        index.index(self)
    }
}

impl<T, I> std::ops::IndexMut<I> for Matrix<T>
where
    I: MatrixIndex<T>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.index_mut(self)
    }
}

/// Any type that implements this trait can be used to index a [`Matrix`].
///
/// # Examples
///
/// ```
/// use matreex::{matrix, Index};
///
/// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
///
/// assert_eq!(matrix[Index::new(1, 1)], 4);
/// assert_eq!(matrix[(1, 1)], 4);
/// assert_eq!(matrix[[1, 1]], 4);
/// ```
pub trait IndexLike {
    /// Returns the row of the index.
    fn row(&self) -> usize;

    /// Returns the column of the index.
    fn col(&self) -> usize;

    /// Returns `true` if the index is out of bounds of given matrix.
    fn is_out_of_bounds_of<T>(&self, matrix: &Matrix<T>) -> bool {
        let shape = matrix.shape();
        self.row() >= shape.nrows || self.col() >= shape.ncols
    }
}

unsafe impl<T, I> MatrixIndex<T> for I
where
    I: IndexLike,
{
    type Output = T;

    fn get(self, matrix: &Matrix<T>) -> Result<&Self::Output> {
        AxisIndex::from_index_with(self, matrix.order).get(matrix)
    }

    fn get_mut(self, matrix: &mut Matrix<T>) -> Result<&mut Self::Output> {
        AxisIndex::from_index_with(self, matrix.order).get_mut(matrix)
    }

    unsafe fn get_unchecked(self, matrix: &Matrix<T>) -> &Self::Output {
        unsafe { AxisIndex::from_index_with(self, matrix.order).get_unchecked(matrix) }
    }

    unsafe fn get_unchecked_mut(self, matrix: &mut Matrix<T>) -> &mut Self::Output {
        unsafe { AxisIndex::from_index_with(self, matrix.order).get_unchecked_mut(matrix) }
    }

    fn index(self, matrix: &Matrix<T>) -> &Self::Output {
        AxisIndex::from_index_with(self, matrix.order).index(matrix)
    }

    fn index_mut(self, matrix: &mut Matrix<T>) -> &mut Self::Output {
        AxisIndex::from_index_with(self, matrix.order).index_mut(matrix)
    }
}

/// A structure that represents the index of an element in a [`Matrix`].
///
/// # Notes
///
/// You might prefer using `(usize, usize)` for matrix indexing instead.
/// Refer to [`IndexLike`] for more information.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Index {
    /// The row index of the element.
    pub row: usize,

    /// The column index of the element.
    pub col: usize,
}

impl Index {
    /// Creates a new [`Index`] instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::Index;
    ///
    /// let index = Index::new(2, 3);
    /// ```
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Creates an `Index` instance from a flattened index unchecked.
    ///
    /// # Notes
    ///
    /// Value returned may be out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{matrix, Index};
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// let index = Index::from_flattened_unchecked_for(4, &matrix);
    /// assert_eq!(index, Index::new(1, 1));
    ///
    /// let index = Index::from_flattened_unchecked_for(6, &matrix);
    /// assert_eq!(index, Index::new(2, 0));
    /// ```
    pub fn from_flattened_unchecked_for<T>(index: usize, matrix: &Matrix<T>) -> Self {
        AxisIndex::from_flattened_unchecked_for(index, matrix.shape).interpret_with(matrix.order)
    }

    /// Creates an `Index` instance from a flattened index.
    ///
    /// # Errors
    ///
    /// - [`Error::IndexOutOfBounds`] if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{matrix, Error, Index};
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// let result = Index::try_from_flattened_for(4, &matrix);
    /// assert_eq!(result, Ok(Index::new(1, 1)));
    ///
    /// let result = Index::try_from_flattened_for(6, &matrix);
    /// assert_eq!(result, Err(Error::IndexOutOfBounds));
    /// ```
    pub fn try_from_flattened_for<T>(index: usize, matrix: &Matrix<T>) -> Result<Self> {
        AxisIndex::try_from_flattened_for(index, matrix.shape)
            .map(|index| index.interpret_with(matrix.order))
    }

    /// Creates an `Index` instance from a flattened index.
    ///
    /// # Panics
    ///
    /// Panics if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{matrix, Index};
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// let index = Index::from_flattened_for(4, &matrix);
    /// assert_eq!(index, Index::new(1, 1));
    /// ```
    ///
    /// ```should_panic
    /// use matreex::{matrix, Index};
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// let index = Index::from_flattened_for(6, &matrix);
    /// ```
    pub fn from_flattened_for<T>(index: usize, matrix: &Matrix<T>) -> Self {
        AxisIndex::from_flattened_for(index, matrix.shape).interpret_with(matrix.order)
    }

    /// Flattens an `Index` instance unchecked.
    ///
    /// # Notes
    ///
    /// Value returned may be out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{matrix, Index};
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// let index = Index::new(1, 1).into_flattened_unchecked_for(&matrix);
    /// assert_eq!(index, 4);
    ///
    /// let index = Index::new(2, 0).into_flattened_unchecked_for(&matrix);
    /// assert_eq!(index, 6);
    /// ```
    pub fn into_flattened_unchecked_for<T>(self, matrix: &Matrix<T>) -> usize {
        AxisIndex::from_index_with(self, matrix.order).into_flattened_unchecked_for(matrix.shape)
    }

    /// Flattens an `Index` instance.
    ///
    /// # Errors
    ///
    /// - [`Error::IndexOutOfBounds`] if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{matrix, Error, Index};
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// let result = Index::new(1, 1).try_into_flattened_for(&matrix);
    /// assert_eq!(result, Ok(4));
    ///
    /// let result = Index::new(2, 0).try_into_flattened_for(&matrix);
    /// assert_eq!(result, Err(Error::IndexOutOfBounds));
    /// ```
    pub fn try_into_flattened_for<T>(self, matrix: &Matrix<T>) -> Result<usize> {
        AxisIndex::from_index_with(self, matrix.order).try_into_flattened_for(matrix.shape)
    }

    /// Flattens an `Index` instance.
    ///
    /// # Panics
    ///
    /// Panics if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{matrix, Index};
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// let index = Index::new(1, 1).into_flattened_for(&matrix);
    /// assert_eq!(index, 4);
    /// ```
    ///
    /// ```should_panic
    /// use matreex::{matrix, Index};
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// let index = Index::new(2, 0).into_flattened_for(&matrix);
    /// ```
    pub fn into_flattened_for<T>(self, matrix: &Matrix<T>) -> usize {
        AxisIndex::from_index_with(self, matrix.order).into_flattened_for(matrix.shape)
    }
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
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
pub(super) struct AxisIndex {
    major: usize,
    minor: usize,
}

impl AxisIndex {
    pub(super) fn new(major: usize, minor: usize) -> Self {
        Self { major, minor }
    }

    pub(super) fn is_out_of_bounds_of(&self, shape: AxisShape) -> bool {
        self.major >= shape.major() || self.minor >= shape.minor()
    }

    pub(super) fn transpose(&mut self) -> &mut Self {
        (self.major, self.minor) = (self.minor, self.major);
        self
    }

    pub(super) fn from_index_with<I: IndexLike>(index: I, order: Order) -> Self {
        let (major, minor) = match order {
            Order::RowMajor => (index.row(), index.col()),
            Order::ColMajor => (index.col(), index.row()),
        };
        Self { major, minor }
    }

    pub(super) fn interpret_with(&self, order: Order) -> Index {
        let (row, col) = match order {
            Order::RowMajor => (self.major, self.minor),
            Order::ColMajor => (self.minor, self.major),
        };
        Index::new(row, col)
    }

    pub(super) fn from_flattened_unchecked_for(index: usize, shape: AxisShape) -> Self {
        let major = index / shape.major_stride();
        // let minor = (index % shape.major_stride()) / shape.minor_stride();
        let minor = index % shape.major_stride();
        Self { major, minor }
    }

    pub(super) fn try_from_flattened_for(index: usize, shape: AxisShape) -> Result<Self> {
        if index >= shape.size() {
            return Err(Error::IndexOutOfBounds);
        }
        Ok(Self::from_flattened_unchecked_for(index, shape))
    }

    pub(super) fn from_flattened_for(index: usize, shape: AxisShape) -> Self {
        match Self::try_from_flattened_for(index, shape) {
            Err(error) => panic!("{error}"),
            Ok(index) => index,
        }
    }

    pub(super) fn into_flattened_unchecked_for(self, shape: AxisShape) -> usize {
        // self.major * shape.major_stride() + self.minor * shape.minor_stride()
        self.major * shape.major_stride() + self.minor
    }

    pub(super) fn try_into_flattened_for(self, shape: AxisShape) -> Result<usize> {
        if self.is_out_of_bounds_of(shape) {
            return Err(Error::IndexOutOfBounds);
        }
        Ok(self.into_flattened_unchecked_for(shape))
    }

    pub(super) fn into_flattened_for(self, shape: AxisShape) -> usize {
        match self.try_into_flattened_for(shape) {
            Err(error) => panic!("{error}"),
            Ok(index) => index,
        }
    }
}

unsafe impl<T> MatrixIndex<T> for AxisIndex {
    type Output = T;

    fn get(self, matrix: &Matrix<T>) -> Result<&Self::Output> {
        let index = self.try_into_flattened_for(matrix.shape)?;
        unsafe { Ok(matrix.data.get_unchecked(index)) }
    }

    fn get_mut(self, matrix: &mut Matrix<T>) -> Result<&mut Self::Output> {
        let index = self.try_into_flattened_for(matrix.shape)?;
        unsafe { Ok(matrix.data.get_unchecked_mut(index)) }
    }

    unsafe fn get_unchecked(self, matrix: &Matrix<T>) -> &Self::Output {
        let index = self.into_flattened_unchecked_for(matrix.shape);
        unsafe { matrix.data.get_unchecked(index) }
    }

    unsafe fn get_unchecked_mut(self, matrix: &mut Matrix<T>) -> &mut Self::Output {
        let index = self.into_flattened_unchecked_for(matrix.shape);
        unsafe { matrix.data.get_unchecked_mut(index) }
    }

    fn index(self, matrix: &Matrix<T>) -> &Self::Output {
        let index = self.into_flattened_for(matrix.shape);
        &matrix.data[index]
    }

    fn index_mut(self, matrix: &mut Matrix<T>) -> &mut Self::Output {
        let index = self.into_flattened_for(matrix.shape);
        &mut matrix.data[index]
    }
}

pub(super) fn translate_index_between_orders_unchecked(
    index: usize,
    src_shape: AxisShape,
) -> usize {
    /*
    This implementation is based on the idea that the element at the same
    position remains the same across different orders. Assuming that the
    original order is `src_order`, and given that the `Index` instance
    representing the position is invariant, we have:

    ```
    let src_flattened_index = index;
    let src_axis_index = AxisIndex::from_flattened_unchecked_for(src_flattened_index, src_shape);

    let position = match src_order {
        Order::RowMajor => Index::new(src_axis_index.major, src_axis_index.minor),
        Order::ColMajor => Index::new(src_axis_index.minor, src_axis_index.major),
    };

    let dest_order = !src_order;
    let dest_axis_index = match dest_order {
        Order::RowMajor => AxisIndex{major: position.row, minor: position.col},
        Order::ColMajor => AxisIndex{major: position.col, minor: position.row},
    };
    let mut dest_shape = src_shape;
    dest_shape.transpose();
    let dest_flattened_index = dest_axis_index.into_flattened_unchecked_for(dest_shape);
    dest_flattened_index
    ```

    Note that `dest_axis_index` is always the transpose of `src_axis_index`,
    which allows us to simplify it to the following implementation:
    */

    let mut index = AxisIndex::from_flattened_unchecked_for(index, src_shape);
    index.transpose();
    let mut dest_shape = src_shape;
    dest_shape.transpose();
    index.into_flattened_unchecked_for(dest_shape)
}

mod internal {
    pub trait Sealed {}

    impl<I: super::IndexLike> Sealed for I {}

    impl Sealed for super::AxisIndex {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn test_get() {
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
    fn test_get_mut() {
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
    fn test_get_unchecked() {
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
    fn test_get_unchecked_mut() {
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
    fn test_index() {
        let matrix = matrix![[0, 1, 2], [3, 4, 5]];
        assert_eq!(matrix[(0, 0)], 0);
        assert_eq!(matrix[(0, 1)], 1);
        assert_eq!(matrix[(0, 2)], 2);
        assert_eq!(matrix[(1, 0)], 3);
        assert_eq!(matrix[(1, 1)], 4);
        assert_eq!(matrix[(1, 2)], 5);
    }

    #[test]
    fn test_index_mut() {
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

    #[test]
    fn test_trait_index_like() {
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
    fn test_struct_index_new() {
        let expected = Index { row: 2, col: 3 };

        assert_eq!(Index::new(2, 3), expected);
        assert_ne!(Index::new(3, 2), expected);
    }

    #[test]
    fn test_struct_index_display() {
        assert_eq!(Index::new(2, 3).to_string(), "(2, 3)");
        assert_eq!(Index::new(3, 2).to_string(), "(3, 2)");
    }
}

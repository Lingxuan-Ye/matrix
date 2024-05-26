//! This module defines [`Matrix`] and all its related components.

pub mod arithmetic;
pub mod index;
pub mod iter;
pub mod order;
pub mod shape;

mod conversion;
mod fmt;
mod operation;

use self::index::{translate_index_between_orders_unchecked, Index};
use self::order::Order;
use self::shape::{AxisShape, IntoAxisShape, Shape, ShapeLike};
use crate::error::{Error, Result};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// [`Matrix`] means matrix.
///
/// Instead of using constructor methods, you may prefer to create a
/// matrix using the [`matrix!`] macro:
///
/// ```
/// use matreex::matrix;
///
/// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
/// ```
///
/// [`matrix!`]: crate::matrix!
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Matrix<T> {
    data: Vec<T>,
    order: Order,
    shape: AxisShape,
}

impl<T: Default> Matrix<T> {
    /// Creates a new [`Matrix`] instance with default values.
    ///
    /// # Panics
    ///
    /// Panics if size exceeds [`usize::MAX`], or total bytes stored
    /// exceeds [`isize::MAX`].
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::Matrix;
    ///
    /// let matrix = Matrix::<i32>::new((2, 3));
    /// ```
    ///
    /// ```should_panic
    /// use matreex::Matrix;
    ///
    /// let matrix = Matrix::<i32>::new((2, usize::MAX));
    /// ```
    ///
    /// ```should_panic
    /// use matreex::Matrix;
    ///
    /// let matrix = Matrix::<i32>::new((1, isize::MAX as usize + 1));
    /// ```
    pub fn new<S: ShapeLike>(shape: S) -> Self {
        match Self::build(shape) {
            Err(error) => panic!("{error}"),
            Ok(matrix) => matrix,
        }
    }

    /// Builds a new [`Matrix`] instance with default values.
    ///
    /// # Errors
    ///
    /// - [`Error::SizeOverflow`] if size exceeds [`usize::MAX`].
    /// - [`Error::CapacityExceeded`] if total bytes stored exceeds [`isize::MAX`].
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{Error, Matrix};
    ///
    /// let result = Matrix::<i32>::build((2, 3));
    /// assert!(result.is_ok());
    ///
    /// let result = Matrix::<i32>::build((2, usize::MAX));
    /// assert_eq!(result, Err(Error::SizeOverflow));
    ///
    /// let result = Matrix::<i32>::build((1, isize::MAX as usize + 1));
    /// assert_eq!(result, Err(Error::CapacityExceeded));
    /// ```
    pub fn build<S: ShapeLike>(shape: S) -> Result<Self> {
        let order = Order::default();
        let shape = shape.try_into_axis_shape(order)?;
        let size = Self::check_size(shape.size())?;
        let data = std::iter::repeat_with(T::default).take(size).collect();
        Ok(Self { data, order, shape })
    }
}

impl<T> Matrix<T> {
    /// Returns the order of the matrix.
    pub fn order(&self) -> Order {
        self.order
    }

    /// Returns the shape of the matrix.
    pub fn shape(&self) -> Shape {
        self.shape.interpret_with(self.order)
    }

    /// Returns the number of rows in the matrix.
    pub fn nrows(&self) -> usize {
        self.shape.interpret_nrows_with(self.order)
    }

    /// Returns the number of columns in the matrix.
    pub fn ncols(&self) -> usize {
        self.shape.interpret_ncols_with(self.order)
    }

    /// Returns the total number of elements in the matrix.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the matrix contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the capacity of the matrix.
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Returns the length of the major axis.
    fn major(&self) -> usize {
        self.shape.major()
    }

    /// Returns the length of the minor axis.
    fn minor(&self) -> usize {
        self.shape.minor()
    }

    /// Returns the stride of the major axis.
    fn major_stride(&self) -> usize {
        self.shape.major_stride()
    }

    /// Returns the stride of the minor axis.
    #[allow(unused)]
    const fn minor_stride(&self) -> usize {
        self.shape.minor_stride()
    }
}

impl<T: Default> Matrix<T> {
    /// Resizes the matrix to the specified shape.
    ///
    /// # Notes
    ///
    /// Reducing the size does not automatically shrink the capacity.
    /// This choice is made to avoid potential reallocation.
    /// Consider explicitly calling [`Matrix::shrink_capacity_to_fit`]
    /// if needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// matrix.resize((2, 2)).unwrap();
    /// assert_eq!(matrix, matrix![[0, 1], [2, 3]]);
    ///
    /// matrix.resize((2, 3)).unwrap();
    /// assert_eq!(matrix, matrix![[0, 1, 2], [3, 0, 0]]);
    /// ```
    pub fn resize<S: ShapeLike>(&mut self, shape: S) -> Result<&mut Self> {
        let shape = shape.try_into_axis_shape(self.order)?;
        let size = Self::check_size(shape.size())?;
        self.data.resize_with(size, T::default);
        self.shape = shape;
        Ok(self)
    }
}

impl<T> Matrix<T> {
    /// Reshapes the matrix to the specified shape.
    ///
    /// # Errors
    ///
    /// - [`Error::SizeMismatch`] if the size of the new shape does not
    /// match the current size of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{matrix, Error};
    ///
    /// let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// matrix.reshape((3, 2)).unwrap();
    /// assert_eq!(matrix, matrix![[0, 1], [2, 3], [4, 5]]);
    ///
    /// let result = matrix.reshape((2, 2));
    /// assert_eq!(result, Err(Error::SizeMismatch));
    /// ```
    pub fn reshape<S: ShapeLike>(&mut self, shape: S) -> Result<&mut Self> {
        match shape.size() {
            Ok(size) if (self.size() == size) => (),
            _ => return Err(Error::SizeMismatch),
        }
        self.shape = shape.into_axis_shape_unchecked(self.order);
        Ok(self)
    }

    /// Transposes the matrix.
    ///
    /// # Notes
    ///
    /// For performance reasons, this method transposes the matrix simply
    /// by changing its order, rather than physically rearranging the data.
    /// This may be considered as having a side effect.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
    /// matrix.transpose();
    /// // column 0
    /// assert_eq!(matrix[(0, 0)], 0);
    /// assert_eq!(matrix[(1, 0)], 1);
    /// assert_eq!(matrix[(2, 0)], 2);
    /// // column 1
    /// assert_eq!(matrix[(0, 1)], 3);
    /// assert_eq!(matrix[(1, 1)], 4);
    /// assert_eq!(matrix[(2, 1)], 5);
    /// ```
    pub fn transpose(&mut self) -> &mut Self {
        self.order = !self.order;
        self
    }

    /// Switches the order of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{matrix, Order};
    ///
    /// let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
    /// assert_eq!(matrix.order(), Order::default());
    ///
    /// matrix.switch_order();
    /// assert_eq!(matrix.order(), !Order::default());
    ///
    /// matrix.switch_order();
    /// assert_eq!(matrix.order(), Order::default());
    /// ```
    pub fn switch_order(&mut self) -> &mut Self {
        let size = self.size();
        let src_shape = self.shape;
        self.shape.transpose();

        let mut visited = vec![false; size];
        for index in 0..size {
            if visited[index] {
                continue;
            }
            let mut current = index;
            while !visited[current] {
                visited[current] = true;
                let next = translate_index_between_orders_unchecked(current, src_shape);
                self.data.swap(index, next);
                current = next;
            }
        }

        self.order = !self.order;
        self
    }

    /// Sets the order of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{matrix, Order};
    ///
    /// let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
    /// assert_eq!(matrix.order(), Order::default());
    ///
    /// matrix.set_order(Order::RowMajor);
    /// assert_eq!(matrix.order(), Order::RowMajor);
    ///
    /// matrix.set_order(Order::ColMajor);
    /// assert_eq!(matrix.order(), Order::ColMajor);
    /// ```
    pub fn set_order(&mut self, order: Order) -> &mut Self {
        if order != self.order {
            self.switch_order();
        }
        self
    }

    /// Shrinks the capacity of the matrix as much as possible.
    pub fn shrink_capacity_to_fit(&mut self) -> &mut Self {
        self.data.shrink_to_fit();
        self
    }

    /// Shrinks the capacity of the matrix with a lower bound.
    ///
    /// The capacity will remain at least as large as both the size
    /// and the supplied value.
    ///
    /// If the current capacity is less than the lower limit,
    /// this is a no-op.
    pub fn shrink_capacity_to(&mut self, min_capacity: usize) -> &mut Self {
        self.data.shrink_to(min_capacity);
        self
    }

    /// Applies a closure to each element of the matrix,
    /// modifying the matrix in place.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// matrix.apply(|x| *x *= 2);
    /// assert_eq!(matrix, matrix![[0, 2, 4], [6, 8, 10]]);
    /// ```
    pub fn apply<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut(&mut T),
    {
        self.data.iter_mut().for_each(f);
        self
    }

    /// Applies a closure to each element of the matrix,
    /// returning a new matrix with the results.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let matrix_i32 = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// let matrix_f64 = matrix_i32.map(|x| *x as f64);
    /// assert_eq!(matrix_f64, matrix![[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]);
    /// ```
    pub fn map<U, F>(&self, f: F) -> Matrix<U>
    where
        F: FnMut(&T) -> U,
    {
        Matrix {
            data: self.data.iter().map(f).collect(),
            order: self.order,
            shape: self.shape,
        }
    }
}

#[cfg(feature = "rayon")]
impl<T> Matrix<T>
where
    T: Sync + Send,
{
    /// Applies a closure to each element of the matrix in parallel,
    /// modifying the matrix in place.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// matrix.par_apply(|x| *x *= 2);
    /// assert_eq!(matrix, matrix![[0, 2, 4], [6, 8, 10]]);
    /// ```
    pub fn par_apply<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(&mut T) + Sync + Send,
    {
        self.data.par_iter_mut().for_each(f);
        self
    }

    /// Applies a closure to each element of the matrix in parallel,
    /// returning a new matrix with the results.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let matrix_i32 = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// let matrix_f64 = matrix_i32.par_map(|x| *x as f64);
    /// assert_eq!(matrix_f64, matrix![[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]);
    /// ```
    pub fn par_map<U, F>(&self, f: F) -> Matrix<U>
    where
        U: Send,
        F: Fn(&T) -> U + Sync + Send,
    {
        Matrix {
            data: self.data.par_iter().map(f).collect(),
            order: self.order,
            shape: self.shape,
        }
    }
}

impl<T: Clone> Matrix<T> {
    /// Overwrites the overlapping part of this matrix with another one,
    /// leaving the non-overlapping part unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{matrix, Order};
    ///
    /// let mut matrix = matrix![[0, 0, 0], [0, 0, 0]];
    /// let other = matrix![[1, 1], [1, 1], [1, 1]];
    ///
    /// matrix.overwrite_with(&other);
    /// assert_eq!(matrix, matrix![[1, 1, 0], [1, 1, 0]]);
    /// ```
    pub fn overwrite_with(&mut self, other: &Self) -> &mut Self {
        if self.order == other.order {
            let major = std::cmp::min(self.major(), other.major());
            let minor = std::cmp::min(self.minor(), other.minor());
            for i in 0..major {
                let self_start = i * self.major_stride();
                let self_end = self_start + minor;
                let other_start = i * other.major_stride();
                let other_end = other_start + minor;
                self.data[self_start..self_end]
                    .clone_from_slice(&other.data[other_start..other_end]);
            }
            return self;
        }

        let self_shape = self.shape();
        let other_shape = other.shape();
        let nrows = std::cmp::min(self_shape.nrows, other_shape.nrows);
        let ncols = std::cmp::min(self_shape.ncols, other_shape.ncols);
        for row in 0..nrows {
            for col in 0..ncols {
                let index = Index::new(row, col);
                unsafe { *self.get_unchecked_mut(index) = other.get_unchecked(index).clone() }
            }
        }
        self
    }
}

impl<T> Matrix<T> {
    fn check_size(size: usize) -> Result<usize> {
        // see more info at https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.with_capacity
        const MAX: usize = isize::MAX as usize;
        match std::mem::size_of::<T>().checked_mul(size) {
            Some(0..=MAX) => Ok(size),
            _ => Err(Error::CapacityExceeded),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    fn shape(major: usize, minor: usize) -> AxisShape {
        AxisShape::build(major, minor).unwrap()
    }

    #[test]
    fn test_new() {
        let expected = matrix![[0, 0, 0], [0, 0, 0]];

        assert_eq!(Matrix::new((2, 3)), expected);
        assert_ne!(Matrix::new((3, 2)), expected);
    }

    #[test]
    fn test_build() {
        let expected = matrix![[0, 0, 0], [0, 0, 0]];

        assert_eq!(Matrix::build((2, 3)).unwrap(), expected);
        assert_ne!(Matrix::build((3, 2)).unwrap(), expected);

        assert_eq!(
            Matrix::<i32>::build((usize::MAX, 2)),
            Err(Error::SizeOverflow)
        );
    }

    #[test]
    fn test_resize() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];

        matrix.resize((2, 3)).unwrap();
        assert_eq!(matrix, matrix![[0, 1, 2], [3, 4, 5]]);

        matrix.resize((2, 2)).unwrap();
        assert_eq!(matrix, matrix![[0, 1], [2, 3]]);

        matrix.resize((3, 3)).unwrap();
        assert_eq!(matrix, matrix![[0, 1, 2], [3, 0, 0], [0, 0, 0]]);

        matrix.resize((2, 3)).unwrap();
        assert_eq!(matrix, matrix![[0, 1, 2], [3, 0, 0]]);

        assert_eq!(matrix.resize((usize::MAX, 2)), Err(Error::SizeOverflow));
        assert_eq!(matrix, matrix![[0, 1, 2], [3, 0, 0]]);

        assert_eq!(
            matrix.resize((isize::MAX as usize + 1, 1)),
            Err(Error::CapacityExceeded)
        );
        assert_eq!(matrix, matrix![[0, 1, 2], [3, 0, 0]]);

        matrix.resize((2, 0)).unwrap();
        assert_eq!(matrix, matrix![[], []]);
    }

    #[test]
    fn test_reshape() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];

        matrix.reshape((2, 3)).unwrap();
        assert_eq!(matrix, matrix![[0, 1, 2], [3, 4, 5]]);

        matrix.reshape((3, 2)).unwrap();
        assert_eq!(matrix, matrix![[0, 1], [2, 3], [4, 5]]);

        matrix.reshape((1, 6)).unwrap();
        assert_eq!(matrix, matrix![[0, 1, 2, 3, 4, 5]]);

        matrix.reshape((6, 1)).unwrap();
        assert_eq!(matrix, matrix![[0], [1], [2], [3], [4], [5]]);

        matrix.reshape((2, 3)).unwrap();
        assert_eq!(matrix, matrix![[0, 1, 2], [3, 4, 5]]);

        assert_eq!(matrix.reshape((usize::MAX, 2)), Err(Error::SizeMismatch));
        assert_eq!(matrix, matrix![[0, 1, 2], [3, 4, 5]]);

        assert_eq!(matrix.reshape((2, 2)), Err(Error::SizeMismatch));
        assert_eq!(matrix, matrix![[0, 1, 2], [3, 4, 5]]);
    }

    #[test]
    fn test_transpose() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];

        matrix.transpose();
        // col 0
        assert_eq!(matrix[(0, 0)], 0);
        assert_eq!(matrix[(1, 0)], 1);
        assert_eq!(matrix[(2, 0)], 2);
        // col 1
        assert_eq!(matrix[(0, 1)], 3);
        assert_eq!(matrix[(1, 1)], 4);
        assert_eq!(matrix[(2, 1)], 5);

        matrix.transpose();
        // row 0
        assert_eq!(matrix[(0, 0)], 0);
        assert_eq!(matrix[(0, 1)], 1);
        assert_eq!(matrix[(0, 2)], 2);
        // row 1
        assert_eq!(matrix[(1, 0)], 3);
        assert_eq!(matrix[(1, 1)], 4);
        assert_eq!(matrix[(1, 2)], 5);
    }

    #[test]
    fn test_switch_order() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
        assert_eq!(matrix.order, Order::RowMajor);
        assert_eq!(matrix.shape, shape(2, 3));

        matrix.switch_order();
        assert_eq!(matrix[(0, 0)], 0);
        assert_eq!(matrix[(0, 1)], 1);
        assert_eq!(matrix[(0, 2)], 2);
        assert_eq!(matrix[(1, 0)], 3);
        assert_eq!(matrix[(1, 1)], 4);
        assert_eq!(matrix[(1, 2)], 5);
        assert_eq!(matrix.order, Order::ColMajor);
        assert_eq!(matrix.shape, shape(3, 2));

        matrix.switch_order();
        assert_eq!(matrix[(0, 0)], 0);
        assert_eq!(matrix[(0, 1)], 1);
        assert_eq!(matrix[(0, 2)], 2);
        assert_eq!(matrix[(1, 0)], 3);
        assert_eq!(matrix[(1, 1)], 4);
        assert_eq!(matrix[(1, 2)], 5);
        assert_eq!(matrix.order, Order::RowMajor);
        assert_eq!(matrix.shape, shape(2, 3));
    }

    #[test]
    fn test_set_order() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
        assert_eq!(matrix.order, Order::RowMajor);
        assert_eq!(matrix.shape, shape(2, 3));

        matrix.set_order(Order::RowMajor);
        assert_eq!(matrix.order, Order::RowMajor);
        assert_eq!(matrix.shape, shape(2, 3));

        matrix.set_order(Order::ColMajor);
        assert_eq!(matrix.order, Order::ColMajor);
        assert_eq!(matrix.shape, shape(3, 2));
    }

    #[test]
    fn test_overwrite_with() {
        let template = matrix![[0, 0, 0], [0, 0, 0]];

        {
            let mut other = matrix![[1, 2]];

            let mut matrix = template.clone();
            matrix.overwrite_with(&other);
            assert_eq!(matrix, matrix![[1, 2, 0], [0, 0, 0]]);

            other.switch_order();

            let mut matrix = template.clone();
            matrix.overwrite_with(&other);
            assert_eq!(matrix, matrix![[1, 2, 0], [0, 0, 0]]);
        }

        {
            let mut other = matrix![[1, 2], [3, 4]];

            let mut matrix = template.clone();
            matrix.overwrite_with(&other);
            assert_eq!(matrix, matrix![[1, 2, 0], [3, 4, 0]]);

            other.switch_order();

            let mut matrix = template.clone();
            matrix.overwrite_with(&other);
            assert_eq!(matrix, matrix![[1, 2, 0], [3, 4, 0]]);
        }

        {
            let mut other = matrix![[1, 2], [3, 4], [5, 6]];

            let mut matrix = template.clone();
            matrix.overwrite_with(&other);
            assert_eq!(matrix, matrix![[1, 2, 0], [3, 4, 0]]);

            other.switch_order();

            let mut matrix = template.clone();
            matrix.overwrite_with(&other);
            assert_eq!(matrix, matrix![[1, 2, 0], [3, 4, 0]]);
        }

        {
            let mut other = matrix![[1, 2, 3]];

            let mut matrix = template.clone();
            matrix.overwrite_with(&other);
            assert_eq!(matrix, matrix![[1, 2, 3], [0, 0, 0]]);

            other.switch_order();

            let mut matrix = template.clone();
            matrix.overwrite_with(&other);
            assert_eq!(matrix, matrix![[1, 2, 3], [0, 0, 0]]);
        }

        {
            let mut other = matrix![[1, 2, 3, 4]];

            let mut matrix = template.clone();
            matrix.overwrite_with(&other);
            assert_eq!(matrix, matrix![[1, 2, 3], [0, 0, 0]]);

            other.switch_order();

            let mut matrix = template.clone();
            matrix.overwrite_with(&other);
            assert_eq!(matrix, matrix![[1, 2, 3], [0, 0, 0]]);
        }
    }

    #[test]
    fn test_check_size() {
        const MAX: usize = isize::MAX as usize;

        assert!(Matrix::<u8>::check_size(MAX).is_ok());
        assert_eq!(
            Matrix::<u8>::check_size(MAX + 1),
            Err(Error::CapacityExceeded)
        );

        assert!(Matrix::<u16>::check_size(MAX / 2).is_ok());
        assert_eq!(
            Matrix::<u16>::check_size(MAX / 2 + 1),
            Err(Error::CapacityExceeded)
        );

        assert!(Matrix::<u32>::check_size(MAX / 4).is_ok());
        assert_eq!(
            Matrix::<u32>::check_size(MAX / 4 + 1),
            Err(Error::CapacityExceeded)
        );

        assert!(Matrix::<u64>::check_size(MAX / 8).is_ok());
        assert_eq!(
            Matrix::<u64>::check_size(MAX / 8 + 1),
            Err(Error::CapacityExceeded)
        );

        assert!(Matrix::<u128>::check_size(MAX / 16).is_ok());
        assert_eq!(
            Matrix::<u128>::check_size(MAX / 16 + 1),
            Err(Error::CapacityExceeded)
        );

        assert!(Matrix::<i8>::check_size(MAX).is_ok());
        assert_eq!(
            Matrix::<i8>::check_size(MAX + 1),
            Err(Error::CapacityExceeded)
        );

        assert!(Matrix::<i16>::check_size(MAX / 2).is_ok());
        assert_eq!(
            Matrix::<i16>::check_size(MAX / 2 + 1),
            Err(Error::CapacityExceeded)
        );

        assert!(Matrix::<i32>::check_size(MAX / 4).is_ok());
        assert_eq!(
            Matrix::<i32>::check_size(MAX / 4 + 1),
            Err(Error::CapacityExceeded)
        );

        assert!(Matrix::<i64>::check_size(MAX / 8).is_ok());
        assert_eq!(
            Matrix::<i64>::check_size(MAX / 8 + 1),
            Err(Error::CapacityExceeded)
        );

        assert!(Matrix::<i128>::check_size(MAX / 16).is_ok());
        assert_eq!(
            Matrix::<i128>::check_size(MAX / 16 + 1),
            Err(Error::CapacityExceeded)
        );

        assert!(Matrix::<bool>::check_size(MAX).is_ok());
        assert_eq!(
            Matrix::<bool>::check_size(MAX + 1),
            Err(Error::CapacityExceeded)
        );

        assert!(Matrix::<char>::check_size(MAX / 4).is_ok());
        assert_eq!(
            Matrix::<char>::check_size(MAX / 4 + 1),
            Err(Error::CapacityExceeded)
        );

        assert!(Matrix::<()>::check_size(MAX).is_ok());
        assert!(Matrix::<()>::check_size(MAX + 1).is_ok());
        assert!(Matrix::<()>::check_size(usize::MAX).is_ok());
    }
}

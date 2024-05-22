use super::super::iter::VectorIter;
use super::super::order::Order;
use super::super::shape::{IntoAxisShape, Shape};
use super::super::Matrix;
use crate::error::{Error, Result};

impl<L> Matrix<L> {
    /// Ensures that two matrices are conformable for multiplication-like
    /// operation.
    ///
    /// # Errors
    ///
    /// - [`Error::NotConformable`] if the matrices are not conformable.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{Error, Matrix};
    ///
    /// let lhs = Matrix::<i32>::new((2, 3));
    ///
    /// let rhs = Matrix::<i32>::new((3, 1));
    /// let result = lhs.ensure_multiplication_like_operation_conformable(&rhs);
    /// assert!(result.is_ok());
    ///
    /// let rhs = Matrix::<i32>::new((2, 3));
    /// let result = lhs.ensure_multiplication_like_operation_conformable(&rhs);
    /// assert_eq!(result, Err(Error::NotConformable));
    /// ```
    pub fn ensure_multiplication_like_operation_conformable<R>(
        &self,
        rhs: &Matrix<R>,
    ) -> Result<&Self> {
        if self.ncols() != rhs.nrows() {
            Err(Error::NotConformable)
        } else {
            Ok(self)
        }
    }

    /// Performs multiplication-like operation on two matrices.
    /// The operation can abort and fill the result with default
    /// values if `op` returns `None` at any point.
    ///
    /// # Errors
    ///
    /// - [`Error::NotConformable`] if the matrices are not conformable.
    ///
    /// # Notes
    ///
    /// The resulting matrix will always have the same order as `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    /// use matreex::matrix::arithmetic::vector_dot_product;
    ///
    /// let lhs = matrix![[0, 1, 2], [3, 4, 5]];
    /// let rhs = matrix![[0, 1], [2, 3], [4, 5]];
    ///
    /// let result = lhs.multiplication_like_operation(&rhs, vector_dot_product);
    /// assert_eq!(result, Ok(matrix![[10, 13], [28, 40]]));
    /// ```
    pub fn multiplication_like_operation<R, F, U>(
        &self,
        rhs: &Matrix<R>,
        mut op: F,
    ) -> Result<Matrix<U>>
    where
        F: FnMut(VectorIter<&L>, VectorIter<&R>) -> Option<U>,
        U: Default,
    {
        self.ensure_multiplication_like_operation_conformable(rhs)?;

        let nrows = self.nrows();
        let ncols = rhs.ncols();
        let order = self.order;
        let shape = Shape::new(nrows, ncols).try_into_axis_shape(order)?;
        let size = shape.size();
        let mut data = Vec::with_capacity(size);
        match order {
            Order::RowMajor => {
                'outer: for row in 0..nrows {
                    for col in 0..ncols {
                        let row_vector = unsafe { self.iter_nth_row_unchecked(row) };
                        let col_vector = unsafe { rhs.iter_nth_col_unchecked(col) };
                        match op(row_vector, col_vector) {
                            None => {
                                data.clear();
                                data.resize_with(size, U::default);
                                break 'outer;
                            }
                            Some(value) => data.push(value),
                        }
                    }
                }
            }
            Order::ColMajor => {
                'outer: for col in 0..ncols {
                    for row in 0..nrows {
                        let row_vector = unsafe { self.iter_nth_row_unchecked(row) };
                        let col_vector = unsafe { rhs.iter_nth_col_unchecked(col) };
                        match op(row_vector, col_vector) {
                            None => {
                                data.clear();
                                data.resize_with(size, U::default);
                                break 'outer;
                            }
                            Some(value) => data.push(value),
                        }
                    }
                }
            }
        }

        Ok(Matrix { data, order, shape })
    }
}

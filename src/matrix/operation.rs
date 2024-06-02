mod arithmetic {
    mod add;
    mod div;
    mod mul;
    mod sub;
}

use super::index::translate_index_between_orders_unchecked;
use super::iter::VectorIter;
use super::order::Order;
use super::shape::{IntoAxisShape, Shape};
use super::Matrix;
use crate::error::{Error, Result};

impl<L> Matrix<L> {
    /// Ensures that two matrices are conformable for elementwise operations.
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
    /// let rhs = Matrix::<i32>::new((2, 3));
    /// let result = lhs.ensure_elementwise_operation_conformable(&rhs);
    /// assert!(result.is_ok());
    ///
    /// let rhs = Matrix::<i32>::new((2, 2));
    /// let result = lhs.ensure_elementwise_operation_conformable(&rhs);
    /// assert_eq!(result, Err(Error::NotConformable));
    /// ```
    pub fn ensure_elementwise_operation_conformable<R>(&self, rhs: &Matrix<R>) -> Result<&Self> {
        if self.shape() != rhs.shape() {
            Err(Error::NotConformable)
        } else {
            Ok(self)
        }
    }

    /// Performs elementwise operation on two matrices.
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
    ///
    /// let lhs = matrix![[0, 1, 2], [3, 4, 5]];
    /// let rhs = matrix![[1, 1, 1], [1, 1, 1]];
    ///
    /// let result = lhs.elementwise_operation(&rhs, |(x, y)| x + y);
    /// assert_eq!(result, Ok(matrix![[1, 2, 3], [4, 5, 6]]));
    /// ```
    pub fn elementwise_operation<R, F, U>(&self, rhs: &Matrix<R>, mut op: F) -> Result<Matrix<U>>
    where
        F: FnMut((&L, &R)) -> U,
    {
        self.ensure_elementwise_operation_conformable(rhs)?;

        let data = if self.order == rhs.order {
            self.data.iter().zip(rhs.data.iter()).map(op).collect()
        } else {
            self.data
                .iter()
                .enumerate()
                .map(|(index, left)| {
                    let index = translate_index_between_orders_unchecked(index, self.shape);
                    let right = unsafe { rhs.data.get_unchecked(index) };
                    op((left, right))
                })
                .collect()
        };

        Ok(Matrix {
            data,
            order: self.order,
            shape: self.shape,
        })
    }

    /// Performs elementwise operation on two matrices, consuming `rhs`.
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
    ///
    /// let lhs = matrix![[0, 1, 2], [3, 4, 5]];
    /// let rhs = matrix![[1, 1, 1], [1, 1, 1]];
    ///
    /// let result = lhs.elementwise_operation_consume_rhs(rhs, |(x, y)| x + y);
    /// assert_eq!(result, Ok(matrix![[1, 2, 3], [4, 5, 6]]));
    /// ```
    pub fn elementwise_operation_consume_rhs<R, F, U>(
        &self,
        rhs: Matrix<R>,
        mut op: F,
    ) -> Result<Matrix<U>>
    where
        R: Clone,
        F: FnMut((&L, R)) -> U,
    {
        self.ensure_elementwise_operation_conformable(&rhs)?;

        let data = if self.order == rhs.order {
            self.data.iter().zip(rhs.data).map(op).collect()
        } else {
            self.data
                .iter()
                .enumerate()
                .map(|(index, left)| {
                    let index = translate_index_between_orders_unchecked(index, self.shape);
                    let right = unsafe { rhs.data.get_unchecked(index) }.clone();
                    op((left, right))
                })
                .collect()
        };

        Ok(Matrix {
            data,
            order: self.order,
            shape: self.shape,
        })
    }

    /// Performs elementwise operation on two matrices, consuming `self`.
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
    ///
    /// let lhs = matrix![[0, 1, 2], [3, 4, 5]];
    /// let rhs = matrix![[1, 1, 1], [1, 1, 1]];
    ///
    /// let result = lhs.elementwise_operation_consume_self(&rhs, |(x, y)| x + y);
    /// assert_eq!(result, Ok(matrix![[1, 2, 3], [4, 5, 6]]));
    /// ```
    pub fn elementwise_operation_consume_self<R, F, U>(
        self,
        rhs: &Matrix<R>,
        mut op: F,
    ) -> Result<Matrix<U>>
    where
        F: FnMut((L, &R)) -> U,
    {
        self.ensure_elementwise_operation_conformable(rhs)?;

        let data = if self.order == rhs.order {
            self.data.into_iter().zip(rhs.data.iter()).map(op).collect()
        } else {
            self.data
                .into_iter()
                .enumerate()
                .map(|(index, left)| {
                    let index = translate_index_between_orders_unchecked(index, self.shape);
                    let right = unsafe { rhs.data.get_unchecked(index) };
                    op((left, right))
                })
                .collect()
        };

        Ok(Matrix {
            data,
            order: self.order,
            shape: self.shape,
        })
    }

    /// Performs elementwise operation on two matrices, consuming both.
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
    ///
    /// let lhs = matrix![[0, 1, 2], [3, 4, 5]];
    /// let rhs = matrix![[1, 1, 1], [1, 1, 1]];
    ///
    /// let result = lhs.elementwise_operation_consume_both(rhs, |(x, y)| x + y);
    /// assert_eq!(result, Ok(matrix![[1, 2, 3], [4, 5, 6]]));
    /// ```
    pub fn elementwise_operation_consume_both<R, F, U>(
        self,
        rhs: Matrix<R>,
        mut op: F,
    ) -> Result<Matrix<U>>
    where
        R: Clone,
        F: FnMut((L, R)) -> U,
    {
        self.ensure_elementwise_operation_conformable(&rhs)?;

        let data = if self.order == rhs.order {
            self.data.into_iter().zip(rhs.data).map(op).collect()
        } else {
            self.data
                .into_iter()
                .enumerate()
                .map(|(index, left)| {
                    let index = translate_index_between_orders_unchecked(index, self.shape);
                    let right = unsafe { rhs.data.get_unchecked(index).clone() };
                    op((left, right))
                })
                .collect()
        };

        Ok(Matrix {
            data,
            order: self.order,
            shape: self.shape,
        })
    }

    /// Performs elementwise operation on two matrices, assigning the result
    /// to `self`.
    ///
    /// # Errors
    ///
    /// - [`Error::NotConformable`] if the matrices are not conformable.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
    /// let rhs = matrix![[1, 1, 1], [1, 1, 1]];
    ///
    /// lhs.elementwise_operation_assign(&rhs, |(x, y)| *x += y).unwrap();
    /// assert_eq!(lhs, matrix![[1, 2, 3], [4, 5, 6]]);
    /// ```
    pub fn elementwise_operation_assign<R, F>(
        &mut self,
        rhs: &Matrix<R>,
        mut op: F,
    ) -> Result<&mut Self>
    where
        F: FnMut((&mut L, &R)),
    {
        self.ensure_elementwise_operation_conformable(rhs)?;

        if self.order == rhs.order {
            self.data.iter_mut().zip(rhs.data.iter()).for_each(op);
        } else {
            self.data.iter_mut().enumerate().for_each(|(index, left)| {
                let index = translate_index_between_orders_unchecked(index, self.shape);
                let right = unsafe { rhs.data.get_unchecked(index) };
                op((left, right))
            });
        }

        Ok(self)
    }

    /// Performs elementwise operation on two matrices, assigning the result
    /// to `self` and consuming `rhs`.
    ///
    /// # Errors
    ///
    /// - [`Error::NotConformable`] if the matrices are not conformable.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
    /// let rhs = matrix![[1, 1, 1], [1, 1, 1]];
    ///
    /// lhs.elementwise_operation_assign_consume_rhs(rhs, |(x, y)| *x += y).unwrap();
    /// assert_eq!(lhs, matrix![[1, 2, 3], [4, 5, 6]]);
    /// ```
    pub fn elementwise_operation_assign_consume_rhs<R, F>(
        &mut self,
        rhs: Matrix<R>,
        mut op: F,
    ) -> Result<&mut Self>
    where
        R: Clone,
        F: FnMut((&mut L, R)),
    {
        self.ensure_elementwise_operation_conformable(&rhs)?;

        if self.order == rhs.order {
            self.data.iter_mut().zip(rhs.data).for_each(op);
        } else {
            self.data.iter_mut().enumerate().for_each(|(index, left)| {
                let index = translate_index_between_orders_unchecked(index, self.shape);
                let right = unsafe { rhs.data.get_unchecked(index) }.clone();
                op((left, right))
            });
        }

        Ok(self)
    }
}

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
    ///
    /// This method is designed to handle **multiplication-like**
    /// operations. For multiplication itself, use [`Matrix::multiply`]
    /// instead, which is more efficient.
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
    /// use matreex::{matrix, VectorIter};
    ///
    /// let lhs = matrix![[0, 1, 2], [3, 4, 5]];
    /// let rhs = matrix![[0, 1], [2, 3], [4, 5]];
    /// let op = |vl: VectorIter<&i32>, vr: VectorIter<&i32>| {
    ///     vl.zip(vr).map(|(x, y)| x * y).reduce(|acc, p| acc + p).unwrap()
    /// };
    ///
    /// let result = lhs.multiplication_like_operation(&rhs, op);
    /// assert_eq!(result, Ok(matrix![[10, 13], [28, 40]]));
    /// ```
    pub fn multiplication_like_operation<R, F, U>(
        &self,
        rhs: &Matrix<R>,
        mut op: F,
    ) -> Result<Matrix<U>>
    where
        F: FnMut(VectorIter<&L>, VectorIter<&R>) -> U,
        U: Default,
    {
        self.ensure_multiplication_like_operation_conformable(rhs)?;

        let nrows = self.nrows();
        let ncols = rhs.ncols();
        let order = self.order;
        let shape = Shape::new(nrows, ncols).try_into_axis_shape(order)?;
        let size = shape.size();
        let mut data = Vec::with_capacity(size);

        if self.ncols() == 0 {
            data.resize_with(size, U::default);
            return Ok(Matrix { data, order, shape });
        }

        match (self.order, rhs.order) {
            (Order::RowMajor, Order::RowMajor) => {
                for row in 0..nrows {
                    for col in 0..ncols {
                        let element = op(
                            unsafe { Box::new(self.iter_nth_major_axis_vector_unchecked(row)) },
                            Box::new(rhs.iter_nth_minor_axis_vector_unchecked(col)),
                        );
                        data.push(element);
                    }
                }
            }

            // best scenario
            (Order::RowMajor, Order::ColMajor) => {
                for row in 0..nrows {
                    for col in 0..ncols {
                        let element = op(
                            unsafe { Box::new(self.iter_nth_major_axis_vector_unchecked(row)) },
                            unsafe { Box::new(rhs.iter_nth_major_axis_vector_unchecked(col)) },
                        );
                        data.push(element);
                    }
                }
            }

            // worst scenario
            (Order::ColMajor, Order::RowMajor) => {
                for col in 0..ncols {
                    for row in 0..nrows {
                        let element = op(
                            Box::new(self.iter_nth_minor_axis_vector_unchecked(row)),
                            Box::new(rhs.iter_nth_minor_axis_vector_unchecked(col)),
                        );
                        data.push(element);
                    }
                }
            }

            (Order::ColMajor, Order::ColMajor) => {
                for col in 0..ncols {
                    for row in 0..nrows {
                        let element = op(
                            Box::new(self.iter_nth_minor_axis_vector_unchecked(row)),
                            unsafe { Box::new(rhs.iter_nth_major_axis_vector_unchecked(col)) },
                        );
                        data.push(element);
                    }
                }
            }
        }

        Ok(Matrix { data, order, shape })
    }
}

impl<T> Matrix<T> {
    /// Performs scalar operation on the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    /// let scalar = 2;
    ///
    /// let result = matrix.scalar_operation(&scalar, |x, y| x + y);
    /// assert_eq!(result, matrix![[2, 3, 4], [5, 6, 7]]);
    /// ```
    pub fn scalar_operation<S, F, U>(&self, scalar: &S, mut op: F) -> Matrix<U>
    where
        F: FnMut(&T, &S) -> U,
    {
        Matrix {
            data: self.data.iter().map(|x| op(x, scalar)).collect(),
            order: self.order,
            shape: self.shape,
        }
    }

    /// Performs scalar operation on the matrix, consuming `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    /// let scalar = 2;
    ///
    /// let result = matrix.scalar_operation_consume_self(&scalar, |x, y| x + y);
    /// assert_eq!(result, matrix![[2, 3, 4], [5, 6, 7]]);
    /// ```
    pub fn scalar_operation_consume_self<S, F, U>(self, scalar: &S, mut op: F) -> Matrix<U>
    where
        F: FnMut(T, &S) -> U,
    {
        Matrix {
            data: self.data.into_iter().map(|x| op(x, scalar)).collect(),
            order: self.order,
            shape: self.shape,
        }
    }

    /// Performs scalar operation on the matrix, assigning the result
    /// to `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
    /// let scalar = 2;
    ///
    /// matrix.scalar_operation_assign(&scalar, |x, y| *x += y);
    /// assert_eq!(matrix, matrix![[2, 3, 4], [5, 6, 7]]);
    /// ```
    pub fn scalar_operation_assign<S, F>(&mut self, scalar: &S, mut op: F) -> &mut Self
    where
        F: FnMut(&mut T, &S),
    {
        self.data.iter_mut().for_each(|x| op(x, scalar));
        self
    }
}

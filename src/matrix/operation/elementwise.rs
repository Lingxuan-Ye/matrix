use super::super::index::translate_index_between_orders_unchecked;
use super::super::Matrix;
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

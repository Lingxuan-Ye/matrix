use super::super::Matrix;

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

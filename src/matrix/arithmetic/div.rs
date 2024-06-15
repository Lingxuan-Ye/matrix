use super::super::Matrix;
use crate::error::Result;
use crate::impl_scalar_div;
use std::ops::{Div, DivAssign};

impl<L> Matrix<L> {
    /// Performs elementwise division on two matrices.
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
    /// let rhs = matrix![[2, 2, 2], [2, 2, 2]];
    ///
    /// let result = lhs.elementwise_div(&rhs);
    /// assert_eq!(result, Ok(matrix![[0, 0, 1], [1, 2, 2]]));
    /// ```
    ///
    /// [`Error::NotConformable`]: crate::error::Error::NotConformable
    pub fn elementwise_div<R, U>(&self, rhs: &Matrix<R>) -> Result<Matrix<U>>
    where
        L: Div<R, Output = U> + Clone,
        R: Clone,
    {
        self.elementwise_operation(rhs, |(left, right)| left.clone() / right.clone())
    }

    /// Performs elementwise division on two matrices, consuming `self`.
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
    /// let rhs = matrix![[2, 2, 2], [2, 2, 2]];
    ///
    /// let result = lhs.elementwise_div_consume_self(&rhs);
    /// assert_eq!(result, Ok(matrix![[0, 0, 1], [1, 2, 2]]));
    /// ```
    ///
    /// [`Error::NotConformable`]: crate::error::Error::NotConformable
    pub fn elementwise_div_consume_self<R, U>(self, rhs: &Matrix<R>) -> Result<Matrix<U>>
    where
        L: Div<R, Output = U>,
        R: Clone,
    {
        self.elementwise_operation_consume_self(rhs, |(left, right)| left / right.clone())
    }

    /// Performs elementwise division on two matrices, assigning the result
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
    /// let rhs = matrix![[2, 2, 2], [2, 2, 2]];
    ///
    /// lhs.elementwise_div_assign(&rhs).unwrap();
    /// assert_eq!(lhs, matrix![[0, 0, 1], [1, 2, 2]]);
    /// ```
    ///
    /// [`Error::NotConformable`]: crate::error::Error::NotConformable
    pub fn elementwise_div_assign<R>(&mut self, rhs: &Matrix<R>) -> Result<&mut Self>
    where
        L: DivAssign<R>,
        R: Clone,
    {
        self.elementwise_operation_assign(rhs, |(left, right)| *left /= right.clone())
    }
}

impl_scalar_div! {u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64}

#[cfg(test)]
mod tests {
    use super::super::super::Matrix;
    use crate::matrix;

    #[test]
    #[allow(clippy::op_ref)]
    fn test_matrix_div_scalar() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = 2;
        let expected = matrix![[0, 0, 1], [1, 2, 2]];

        {
            let result = &lhs / &rhs;
            assert_eq!(result, expected);

            let result = &lhs / rhs;
            assert_eq!(result, expected);

            let result = lhs.clone() / &rhs;
            assert_eq!(result, expected);

            let result = lhs.clone() / rhs;
            assert_eq!(result, expected);
        }

        {
            lhs.switch_order();

            let mut result: Matrix<i32> = &lhs / &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = &lhs / rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = lhs.clone() / &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = lhs.clone() / rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn test_scalar_div_matrix() {
        let lhs = 12;
        let mut rhs = matrix![[1, 2, 3], [4, 5, 6]];
        let expected = matrix![[12, 6, 4], [3, 2, 2]];

        {
            let result = &lhs / &rhs;
            assert_eq!(result, expected);

            let result = lhs / &rhs;
            assert_eq!(result, expected);

            let result = &lhs / rhs.clone();
            assert_eq!(result, expected);

            let result = lhs / rhs.clone();
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let mut result: Matrix<i32> = &lhs / &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = lhs / &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = &lhs / rhs.clone();
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = lhs / rhs.clone();
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_matrix_div_scalar_assign() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = 2;
        let expected = matrix![[0, 0, 1], [1, 2, 2]];

        {
            let mut result = lhs.clone();
            result /= &rhs;
            assert_eq!(result, expected);

            let mut result = lhs.clone();
            result /= rhs;
            assert_eq!(result, expected);
        }

        {
            lhs.switch_order();

            let mut result = lhs.clone();
            result /= &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result = lhs.clone();
            result /= rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }
}

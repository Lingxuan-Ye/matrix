use super::super::Matrix;
use crate::error::Result;
use crate::impl_scalar_sub;
use std::ops::{Sub, SubAssign};

impl<L, R, U> Sub<Matrix<R>> for Matrix<L>
where
    L: Sub<R, Output = U>,
    R: Clone,
{
    type Output = Matrix<U>;

    fn sub(self, rhs: Matrix<R>) -> Self::Output {
        self.sub(&rhs)
    }
}

impl<L, R, U> Sub<&Matrix<R>> for Matrix<L>
where
    L: Sub<R, Output = U>,
    R: Clone,
{
    type Output = Matrix<U>;

    fn sub(self, rhs: &Matrix<R>) -> Self::Output {
        match self.elementwise_sub_consume_self(rhs) {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, U> Sub<Matrix<R>> for &Matrix<L>
where
    L: Sub<R, Output = U> + Clone,
    R: Clone,
{
    type Output = Matrix<U>;

    fn sub(self, rhs: Matrix<R>) -> Self::Output {
        self.sub(&rhs)
    }
}

impl<L, R, U> Sub<&Matrix<R>> for &Matrix<L>
where
    L: Sub<R, Output = U> + Clone,
    R: Clone,
{
    type Output = Matrix<U>;

    fn sub(self, rhs: &Matrix<R>) -> Self::Output {
        match self.elementwise_sub(rhs) {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R> SubAssign<Matrix<R>> for Matrix<L>
where
    L: SubAssign<R>,
    R: Clone,
{
    fn sub_assign(&mut self, rhs: Matrix<R>) {
        self.sub_assign(&rhs)
    }
}

impl<L, R> SubAssign<&Matrix<R>> for Matrix<L>
where
    L: SubAssign<R>,
    R: Clone,
{
    fn sub_assign(&mut self, rhs: &Matrix<R>) {
        if let Err(error) = self.elementwise_sub_assign(rhs) {
            panic!("{error}");
        }
    }
}

impl<L> Matrix<L> {
    /// Performs elementwise subtraction on two matrices.
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
    /// let result = lhs.elementwise_sub(&rhs);
    /// assert_eq!(result, Ok(matrix![[-2, -1, 0], [1, 2, 3]]));
    /// ```
    ///
    /// [`Error::NotConformable`]: crate::error::Error::NotConformable
    pub fn elementwise_sub<R, U>(&self, rhs: &Matrix<R>) -> Result<Matrix<U>>
    where
        L: Sub<R, Output = U> + Clone,
        R: Clone,
    {
        self.elementwise_operation(rhs, |(left, right)| left.clone() - right.clone())
    }

    /// Performs elementwise subtraction on two matrices, consuming `self`.
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
    /// let result = lhs.elementwise_sub_consume_self(&rhs);
    /// assert_eq!(result, Ok(matrix![[-2, -1, 0], [1, 2, 3]]));
    /// ```
    ///
    /// [`Error::NotConformable`]: crate::error::Error::NotConformable
    pub fn elementwise_sub_consume_self<R, U>(self, rhs: &Matrix<R>) -> Result<Matrix<U>>
    where
        L: Sub<R, Output = U>,
        R: Clone,
    {
        self.elementwise_operation_consume_self(rhs, |(left, right)| left - right.clone())
    }

    /// Performs elementwise subtraction on two matrices, assigning the result
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
    /// lhs.elementwise_sub_assign(&rhs).unwrap();
    /// assert_eq!(lhs, matrix![[-2, -1, 0], [1, 2, 3]]);
    /// ```
    ///
    /// [`Error::NotConformable`]: crate::error::Error::NotConformable
    pub fn elementwise_sub_assign<R>(&mut self, rhs: &Matrix<R>) -> Result<&mut Self>
    where
        L: SubAssign<R>,
        R: Clone,
    {
        self.elementwise_operation_assign(rhs, |(left, right)| *left -= right.clone())
    }
}

impl_scalar_sub! {u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn test_sub() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4, 3], [2, 1, 0]];
        let expected = matrix![[-5, -3, -1], [1, 3, 5]];

        {
            let result = &lhs - &rhs;
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let result = &lhs - &rhs;
            assert_eq!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);

            rhs.switch_order();
        }

        {
            lhs.switch_order();

            let mut result = &lhs - &rhs;
            assert_ne!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_sub_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[1, 1], [2, 2], [3, 3]];

        let _ = &lhs - &rhs;
    }

    #[test]
    fn test_sub_consume_rhs() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4, 3], [2, 1, 0]];
        let expected = matrix![[-5, -3, -1], [1, 3, 5]];

        {
            let result = &lhs - rhs.clone();
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let result = &lhs - rhs.clone();
            assert_eq!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);

            rhs.switch_order();
        }

        {
            lhs.switch_order();

            let mut result = &lhs - rhs.clone();
            assert_ne!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_sub_consume_rhs_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[1, 1], [2, 2], [3, 3]];

        let _ = &lhs - rhs;
    }

    #[test]
    fn test_sub_consume_lhs() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4, 3], [2, 1, 0]];
        let expected = matrix![[-5, -3, -1], [1, 3, 5]];

        {
            let result = lhs.clone() - &rhs;
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let result = lhs.clone() - &rhs;
            assert_eq!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);

            rhs.switch_order();
        }

        {
            lhs.switch_order();

            let mut result = lhs.clone() - &rhs;
            assert_ne!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_sub_consume_lhs_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[1, 1], [2, 2], [3, 3]];

        let _ = lhs - &rhs;
    }

    #[test]
    fn test_sub_consume_both() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4, 3], [2, 1, 0]];
        let expected = matrix![[-5, -3, -1], [1, 3, 5]];

        {
            let result = lhs.clone() - rhs.clone();
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let result = lhs.clone() - rhs.clone();
            assert_eq!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);

            rhs.switch_order();
        }

        {
            lhs.switch_order();

            let mut result = lhs.clone() - rhs.clone();
            assert_ne!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_sub_consume_both_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[1, 1], [2, 2], [3, 3]];

        let _ = lhs - rhs;
    }

    #[test]
    fn test_sub_assign() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4, 3], [2, 1, 0]];
        let expected = matrix![[-5, -3, -1], [1, 3, 5]];

        {
            let mut result = lhs.clone();
            result -= &rhs;
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let mut result = lhs.clone();
            result -= &rhs;
            assert_eq!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);

            rhs.switch_order();
        }

        {
            lhs.switch_order();

            let mut result = lhs.clone();
            result -= &rhs;
            assert_ne!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_sub_assign_fails() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[1, 1], [2, 2], [3, 3]];

        lhs -= &rhs;
    }

    #[test]
    fn test_sub_assign_consume_rhs() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4, 3], [2, 1, 0]];
        let expected = matrix![[-5, -3, -1], [1, 3, 5]];

        {
            let mut result = lhs.clone();
            result -= rhs.clone();
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let mut result = lhs.clone();
            result -= rhs.clone();
            assert_eq!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);

            rhs.switch_order();
        }

        {
            lhs.switch_order();

            let mut result = lhs.clone();
            result -= rhs.clone();
            assert_ne!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_sub_assign_consume_rhs_fails() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[1, 1], [2, 2], [3, 3]];

        lhs -= rhs;
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn test_matrix_sub_scalar() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = 2;
        let expected = matrix![[-2, -1, 0], [1, 2, 3]];

        {
            let result = &lhs - &rhs;
            assert_eq!(result, expected);

            let result = &lhs - rhs;
            assert_eq!(result, expected);

            let result = lhs.clone() - &rhs;
            assert_eq!(result, expected);

            let result = lhs.clone() - rhs;
            assert_eq!(result, expected);
        }

        {
            lhs.switch_order();

            let mut result: Matrix<i32> = &lhs - &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = &lhs - rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = lhs.clone() - &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = lhs.clone() - rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn test_scalar_sub_matrix() {
        let lhs = 2;
        let mut rhs = matrix![[0, 1, 2], [3, 4, 5]];
        let expected = matrix![[2, 1, 0], [-1, -2, -3]];

        {
            let result = &lhs - &rhs;
            assert_eq!(result, expected);

            let result = lhs - &rhs;
            assert_eq!(result, expected);

            let result = &lhs - rhs.clone();
            assert_eq!(result, expected);

            let result = lhs - rhs.clone();
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let mut result: Matrix<i32> = &lhs - &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = lhs - &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = &lhs - rhs.clone();
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = lhs - rhs.clone();
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_matrix_sub_scalar_assign() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = 2;
        let expected = matrix![[-2, -1, 0], [1, 2, 3]];

        {
            let mut result = lhs.clone();
            result -= &rhs;
            assert_eq!(result, expected);

            let mut result = lhs.clone();
            result -= rhs;
            assert_eq!(result, expected);
        }

        {
            lhs.switch_order();

            let mut result = lhs.clone();
            result -= &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result = lhs.clone();
            result -= rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }
}

use super::super::super::Matrix;
use std::ops::{Sub, SubAssign};

impl<L, R, U> Sub<&Matrix<R>> for &Matrix<L>
where
    L: Sub<R, Output = U> + Clone,
    R: Clone,
{
    type Output = Matrix<U>;

    fn sub(self, rhs: &Matrix<R>) -> Self::Output {
        let result = self.elementwise_operation(rhs, |(left, right)| left.clone() - right.clone());
        match result {
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
        let result =
            self.elementwise_operation_consume_rhs(rhs, |(left, right)| left.clone() - right);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, U> Sub<&Matrix<R>> for Matrix<L>
where
    L: Sub<R, Output = U>,
    R: Clone,
{
    type Output = Matrix<U>;

    fn sub(self, rhs: &Matrix<R>) -> Self::Output {
        let result =
            self.elementwise_operation_consume_self(rhs, |(left, right)| left - right.clone());
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, U> Sub<Matrix<R>> for Matrix<L>
where
    L: Sub<R, Output = U>,
    R: Clone,
{
    type Output = Matrix<U>;

    fn sub(self, rhs: Matrix<R>) -> Self::Output {
        let result = self.elementwise_operation_consume_both(rhs, |(left, right)| left - right);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R> SubAssign<&Matrix<R>> for Matrix<L>
where
    L: SubAssign<R>,
    R: Clone,
{
    fn sub_assign(&mut self, rhs: &Matrix<R>) {
        let result = self.elementwise_operation_assign(rhs, |(left, right)| *left -= right.clone());
        if let Err(error) = result {
            panic!("{error}");
        }
    }
}

impl<L, R> SubAssign<Matrix<R>> for Matrix<L>
where
    L: SubAssign<R>,
    R: Clone,
{
    fn sub_assign(&mut self, rhs: Matrix<R>) {
        let result =
            self.elementwise_operation_assign_consume_rhs(rhs, |(left, right)| *left -= right);
        if let Err(error) = result {
            panic!("{error}");
        }
    }
}

/// Implements scalar subtraction for [`Matrix`].
///
/// # Notes
///
/// Refer to [`impl_scalar_add!`] for more information.
///
/// [`impl_scalar_add!`]: crate::impl_scalar_add!
#[macro_export]
macro_rules! impl_scalar_sub {
    ($($t:ty)*) => {
        $(
            impl std::ops::Sub<&$t> for &$crate::matrix::Matrix<&$t>
            where
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation(rhs, |element, scalar| (*element).clone() - scalar.clone())
                }
            }

            impl std::ops::Sub<$t> for &$crate::matrix::Matrix<&$t>
            where
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: $t) -> Self::Output {
                    self.scalar_operation(&rhs, |element, scalar| (*element).clone() - scalar.clone())
                }
            }

            impl std::ops::Sub<&$t> for $crate::matrix::Matrix<&$t>
            where
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation_consume_self(rhs, |element, scalar| element.clone() - scalar.clone())
                }
            }

            impl std::ops::Sub<$t> for $crate::matrix::Matrix<&$t>
            where
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: $t) -> Self::Output {
                    self.scalar_operation_consume_self(&rhs, |element, scalar| element.clone() - scalar.clone())
                }
            }

            impl std::ops::Sub<&$t> for &$crate::matrix::Matrix<$t>
            where
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation(rhs, |element, scalar| element.clone() - scalar.clone())
                }
            }

            impl std::ops::Sub<$t> for &$crate::matrix::Matrix<$t>
            where
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: $t) -> Self::Output {
                    self.scalar_operation(&rhs, |element, scalar| element.clone() - scalar.clone())
                }
            }

            impl std::ops::Sub<&$t> for $crate::matrix::Matrix<$t>
            where
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation_consume_self(rhs, |element, scalar| element - scalar.clone())
                }
            }

            impl std::ops::Sub<$t> for $crate::matrix::Matrix<$t>
            where
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: $t) -> Self::Output {
                    self.scalar_operation_consume_self(&rhs, |element, scalar| element - scalar.clone())
                }
            }

            impl std::ops::Sub<&$crate::matrix::Matrix<&$t>> for &$t
            where
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: &$crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation(self, |element, scalar| scalar.clone() - (*element).clone())
                }
            }

            impl std::ops::Sub<$crate::matrix::Matrix<&$t>> for &$t
            where
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: $crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(self, |element, scalar| scalar.clone() - element.clone())
                }
            }

            impl std::ops::Sub<&$crate::matrix::Matrix<$t>> for &$t
            where
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: &$crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation(self, |element, scalar| scalar.clone() - element.clone())
                }
            }

            impl std::ops::Sub<$crate::matrix::Matrix<$t>> for &$t
            where
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: $crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(self, |element, scalar| scalar.clone() - element)
                }
            }

            impl std::ops::Sub<&$crate::matrix::Matrix<&$t>> for $t
            where
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: &$crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation(&self, |element, scalar| scalar.clone() - (*element).clone())
                }
            }

            impl std::ops::Sub<$crate::matrix::Matrix<&$t>> for $t
            where
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: $crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(&self, |element, scalar| scalar.clone() - element.clone())
                }
            }

            impl std::ops::Sub<&$crate::matrix::Matrix<$t>> for $t
            where
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: &$crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation(&self, |element, scalar| scalar.clone() - element.clone())
                }
            }

            impl std::ops::Sub<$crate::matrix::Matrix<$t>> for $t
            where
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: $crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(&self, |element, scalar| scalar.clone() - element)
                }
            }

            impl std::ops::SubAssign<&$t> for $crate::matrix::Matrix<$t>
            where
                $t: Clone,
            {
                fn sub_assign(&mut self, rhs: &$t) {
                    self.scalar_operation_assign(rhs, |element, scalar| *element -= scalar.clone());
                }
            }

            impl std::ops::SubAssign<$t> for $crate::matrix::Matrix<$t>
            where
                $t: Clone,
            {
                fn sub_assign(&mut self, rhs: $t) {
                    self.scalar_operation_assign(&rhs, |element, scalar| *element -= scalar.clone());
                }
            }
        )*
    }
}

impl_scalar_sub!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

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

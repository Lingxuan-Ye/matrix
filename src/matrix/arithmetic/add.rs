use super::super::Matrix;
use crate::impl_scalar_add;
use std::ops::{Add, AddAssign};

impl<L, R, U> Add<&Matrix<R>> for &Matrix<L>
where
    L: Add<R, Output = U> + Clone,
    R: Clone,
{
    type Output = Matrix<U>;

    fn add(self, rhs: &Matrix<R>) -> Self::Output {
        let result = self.elementwise_operation(rhs, |(left, right)| left.clone() + right.clone());
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, U> Add<Matrix<R>> for &Matrix<L>
where
    L: Add<R, Output = U> + Clone,
    R: Clone,
{
    type Output = Matrix<U>;

    fn add(self, rhs: Matrix<R>) -> Self::Output {
        let result =
            self.elementwise_operation_consume_rhs(rhs, |(left, right)| left.clone() + right);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, U> Add<&Matrix<R>> for Matrix<L>
where
    L: Add<R, Output = U>,
    R: Clone,
{
    type Output = Matrix<U>;

    fn add(self, rhs: &Matrix<R>) -> Self::Output {
        let result =
            self.elementwise_operation_consume_self(rhs, |(left, right)| left + right.clone());
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, U> Add<Matrix<R>> for Matrix<L>
where
    L: Add<R, Output = U>,
    R: Clone,
{
    type Output = Matrix<U>;

    fn add(self, rhs: Matrix<R>) -> Self::Output {
        let result = self.elementwise_operation_consume_both(rhs, |(left, right)| left + right);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R> AddAssign<&Matrix<R>> for Matrix<L>
where
    L: AddAssign<R>,
    R: Clone,
{
    fn add_assign(&mut self, rhs: &Matrix<R>) {
        let result = self.elementwise_operation_assign(rhs, |(left, right)| *left += right.clone());
        if let Err(error) = result {
            panic!("{error}");
        }
    }
}

impl<L, R> AddAssign<Matrix<R>> for Matrix<L>
where
    L: AddAssign<R>,
    R: Clone,
{
    fn add_assign(&mut self, rhs: Matrix<R>) {
        let result =
            self.elementwise_operation_assign_consume_rhs(rhs, |(left, right)| *left += right);
        if let Err(error) = result {
            panic!("{error}");
        }
    }
}

/*
The `Clone` trait is specified for the concrete type `$t` to ensure that
`scalar.clone` does not produce a reference but an owned object. This helps
to generate a more intuitive error message when the bound is not satisfied.
*/

/// Implements scalar addition for [`Matrix`].
///
/// # Notes
///
/// A `scalar` does not have to be a scalar in the mathematical sense. Instead,
/// it can be any type except for [`Matrix`]. However, if you do need to treat
/// some concrete type of [`Matrix`] as a scalar, you can wrap it in a newtype
/// and implement all the necessary trait bounds for it.
#[macro_export]
macro_rules! impl_scalar_add {
    ($($t:ty)*) => {
        $(
            impl<L, U> std::ops::Add<&$t> for &$crate::matrix::Matrix<L>
            where
                L: std::ops::Add<$t, Output = U> + Clone,
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<U>;

                fn add(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation(rhs, |element, scalar| element.clone() + scalar.clone())
                }
            }

            impl<L, U> std::ops::Add<$t> for &$crate::matrix::Matrix<L>
            where
                L: std::ops::Add<$t, Output = U> + Clone,
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<U>;

                fn add(self, rhs: $t) -> Self::Output {
                    self.scalar_operation(&rhs, |element, scalar| element.clone() + scalar.clone())
                }
            }

            impl<L, U> std::ops::Add<&$t> for $crate::matrix::Matrix<L>
            where
                L: std::ops::Add<$t, Output = U>,
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<U>;

                fn add(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation_consume_self(rhs, |element, scalar| element + scalar.clone())
                }
            }

            impl<L, U> std::ops::Add<$t> for $crate::matrix::Matrix<L>
            where
                L: std::ops::Add<$t, Output = U>,
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<U>;

                fn add(self, rhs: $t) -> Self::Output {
                    self.scalar_operation_consume_self(&rhs, |element, scalar| element + scalar.clone())
                }
            }

            impl<R, U> std::ops::Add<&$crate::matrix::Matrix<R>> for &$t
            where
                $t: std::ops::Add<R, Output = U> + Clone,
                R: Clone,
            {
                type Output = $crate::matrix::Matrix<U>;

                fn add(self, rhs: &$crate::matrix::Matrix<R>) -> Self::Output {
                    rhs.scalar_operation(self, |element, scalar| scalar.clone() + element.clone())
                }
            }

            impl<R, U> std::ops::Add<$crate::matrix::Matrix<R>> for &$t
            where
                $t: std::ops::Add<R, Output = U> + Clone,
                R: Clone,
            {
                type Output = $crate::matrix::Matrix<U>;

                fn add(self, rhs: $crate::matrix::Matrix<R>) -> Self::Output {
                    rhs.scalar_operation_consume_self(self, |element, scalar| scalar.clone() + element.clone())
                }
            }

            impl<R, U> std::ops::Add<&$crate::matrix::Matrix<R>> for $t
            where
                $t: std::ops::Add<R, Output = U> + Clone,
                R: Clone,
            {
                type Output = $crate::matrix::Matrix<U>;

                fn add(self, rhs: &$crate::matrix::Matrix<R>) -> Self::Output {
                    rhs.scalar_operation(&self, |element, scalar| scalar.clone() + element.clone())
                }
            }

            impl<R, U> std::ops::Add<$crate::matrix::Matrix<R>> for $t
            where
                $t: std::ops::Add<R, Output = U> + Clone,
                R: Clone,
            {
                type Output = $crate::matrix::Matrix<U>;

                fn add(self, rhs: $crate::matrix::Matrix<R>) -> Self::Output {
                    rhs.scalar_operation_consume_self(&self, |element, scalar| scalar.clone() + element.clone())
                }
            }

            impl<L> std::ops::AddAssign<&$t> for $crate::matrix::Matrix<L>
            where
                L: std::ops::AddAssign<$t>,
                $t: Clone,
            {
                fn add_assign(&mut self, rhs: &$t) {
                    self.scalar_operation_assign(rhs, |element, scalar| *element += scalar.clone());
                }
            }

            impl<L> std::ops::AddAssign<$t> for $crate::matrix::Matrix<L>
            where
                L: std::ops::AddAssign<$t>,
                $t: Clone,
            {
                fn add_assign(&mut self, rhs: $t) {
                    self.scalar_operation_assign(&rhs, |element, scalar| *element += scalar.clone());
                }
            }
        )*
    }
}

impl_scalar_add!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn test_add() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4, 3], [2, 1, 0]];
        let expected = matrix![[5, 5, 5], [5, 5, 5]];

        {
            let result = &lhs + &rhs;
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let result = &lhs + &rhs;
            assert_eq!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);

            rhs.switch_order();
        }

        {
            lhs.switch_order();

            let mut result = &lhs + &rhs;
            assert_ne!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_add_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[1, 1], [2, 2], [3, 3]];

        let _ = &lhs + &rhs;
    }

    #[test]
    fn test_add_consume_rhs() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4, 3], [2, 1, 0]];
        let expected = matrix![[5, 5, 5], [5, 5, 5]];

        {
            let result = &lhs + rhs.clone();
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let result = &lhs + rhs.clone();
            assert_eq!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);

            rhs.switch_order();
        }

        {
            lhs.switch_order();

            let mut result = &lhs + rhs.clone();
            assert_ne!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_add_consume_rhs_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[1, 1], [2, 2], [3, 3]];

        let _ = &lhs + rhs;
    }

    #[test]
    fn test_add_consume_lhs() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4, 3], [2, 1, 0]];
        let expected = matrix![[5, 5, 5], [5, 5, 5]];

        {
            let result = lhs.clone() + &rhs;
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let result = lhs.clone() + &rhs;
            assert_eq!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);

            rhs.switch_order();
        }

        {
            lhs.switch_order();

            let mut result = lhs.clone() + &rhs;
            assert_ne!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_add_consume_lhs_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[1, 1], [2, 2], [3, 3]];

        let _ = lhs + &rhs;
    }

    #[test]
    fn test_add_consume_both() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4, 3], [2, 1, 0]];
        let expected = matrix![[5, 5, 5], [5, 5, 5]];

        {
            let result = lhs.clone() + rhs.clone();
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let result = lhs.clone() + rhs.clone();
            assert_eq!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);

            rhs.switch_order();
        }

        {
            lhs.switch_order();

            let mut result = lhs.clone() + rhs.clone();
            assert_ne!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_add_consume_both_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[1, 1], [2, 2], [3, 3]];

        let _ = lhs + rhs;
    }

    #[test]
    fn test_add_assign() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4, 3], [2, 1, 0]];
        let expected = matrix![[5, 5, 5], [5, 5, 5]];

        {
            let mut result = lhs.clone();
            result += &rhs;
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let mut result = lhs.clone();
            result += &rhs;
            assert_eq!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);

            rhs.switch_order();
        }

        {
            lhs.switch_order();

            let mut result = lhs.clone();
            result += &rhs;
            assert_ne!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_add_assign_fails() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[1, 1], [2, 2], [3, 3]];

        lhs += &rhs;
    }

    #[test]
    fn test_add_assign_consume_rhs() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4, 3], [2, 1, 0]];
        let expected = matrix![[5, 5, 5], [5, 5, 5]];

        {
            let mut result = lhs.clone();
            result += rhs.clone();
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let mut result = lhs.clone();
            result += rhs.clone();
            assert_eq!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);

            rhs.switch_order();
        }

        {
            lhs.switch_order();

            let mut result = lhs.clone();
            result += rhs.clone();
            assert_ne!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_add_assign_consume_rhs_fails() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[1, 1], [2, 2], [3, 3]];

        lhs += rhs;
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn test_matrix_add_scalar() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = 2;
        let expected = matrix![[2, 3, 4], [5, 6, 7]];

        {
            let result = &lhs + &rhs;
            assert_eq!(result, expected);

            let result = &lhs + rhs;
            assert_eq!(result, expected);

            let result = lhs.clone() + &rhs;
            assert_eq!(result, expected);

            let result = lhs.clone() + rhs;
            assert_eq!(result, expected);
        }

        {
            lhs.switch_order();

            let mut result: Matrix<i32> = &lhs + &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = &lhs + rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = lhs.clone() + &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = lhs.clone() + rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn test_scalar_add_matrix() {
        let lhs = 2;
        let mut rhs = matrix![[0, 1, 2], [3, 4, 5]];
        let expected = matrix![[2, 3, 4], [5, 6, 7]];

        {
            let result = &lhs + &rhs;
            assert_eq!(result, expected);

            let result = lhs + &rhs;
            assert_eq!(result, expected);

            let result = &lhs + rhs.clone();
            assert_eq!(result, expected);

            let result = lhs + rhs.clone();
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let mut result: Matrix<i32> = &lhs + &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = lhs + &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = &lhs + rhs.clone();
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = lhs + rhs.clone();
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_matrix_add_scalar_assign() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = 2;
        let expected = matrix![[2, 3, 4], [5, 6, 7]];

        {
            let mut result = lhs.clone();
            result += &rhs;
            assert_eq!(result, expected);

            let mut result = lhs.clone();
            result += rhs;
            assert_eq!(result, expected);
        }

        {
            lhs.switch_order();

            let mut result = lhs.clone();
            result += &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result = lhs.clone();
            result += rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }
}

/// Implements scalar division for [`Matrix`].
///
/// # Notes
///
/// Refer to [`impl_scalar_add!`] for more information.
///
/// [`Matrix`]: crate::matrix::Matrix
/// [`impl_scalar_add!`]: crate::impl_scalar_add!
#[macro_export]
macro_rules! impl_scalar_div {
    ($($t:ty)*) => {
        $(
            impl<L, U> std::ops::Div<&$t> for &$crate::matrix::Matrix<L>
            where
                L: std::ops::Div<$t, Output = U> + Clone,
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<U>;

                fn div(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation(rhs, |element, scalar| element.clone() / scalar.clone())
                }
            }

            impl<L, U> std::ops::Div<$t> for &$crate::matrix::Matrix<L>
            where
                L: std::ops::Div<$t, Output = U> + Clone,
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<U>;

                fn div(self, rhs: $t) -> Self::Output {
                    self.scalar_operation(&rhs, |element, scalar| element.clone() / scalar.clone())
                }
            }

            impl<L, U> std::ops::Div<&$t> for $crate::matrix::Matrix<L>
            where
                L: std::ops::Div<$t, Output = U>,
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<U>;

                fn div(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation_consume_self(rhs, |element, scalar| element / scalar.clone())
                }
            }

            impl<L, U> std::ops::Div<$t> for $crate::matrix::Matrix<L>
            where
                L: std::ops::Div<$t, Output = U>,
                $t: Clone,
            {
                type Output = $crate::matrix::Matrix<U>;

                fn div(self, rhs: $t) -> Self::Output {
                    self.scalar_operation_consume_self(&rhs, |element, scalar| element / scalar.clone())
                }
            }

            impl<R, U> std::ops::Div<&$crate::matrix::Matrix<R>> for &$t
            where
                $t: std::ops::Div<R, Output = U> + Clone,
                R: Clone,
            {
                type Output = $crate::matrix::Matrix<U>;

                fn div(self, rhs: &$crate::matrix::Matrix<R>) -> Self::Output {
                    rhs.scalar_operation(self, |element, scalar| scalar.clone() / element.clone())
                }
            }

            impl<R, U> std::ops::Div<$crate::matrix::Matrix<R>> for &$t
            where
                $t: std::ops::Div<R, Output = U> + Clone,
                R: Clone,
            {
                type Output = $crate::matrix::Matrix<U>;

                fn div(self, rhs: $crate::matrix::Matrix<R>) -> Self::Output {
                    rhs.scalar_operation_consume_self(self, |element, scalar| scalar.clone() / element.clone())
                }
            }

            impl<R, U> std::ops::Div<&$crate::matrix::Matrix<R>> for $t
            where
                $t: std::ops::Div<R, Output = U> + Clone,
                R: Clone,
            {
                type Output = $crate::matrix::Matrix<U>;

                fn div(self, rhs: &$crate::matrix::Matrix<R>) -> Self::Output {
                    rhs.scalar_operation(&self, |element, scalar| scalar.clone() / element.clone())
                }
            }

            impl<R, U> std::ops::Div<$crate::matrix::Matrix<R>> for $t
            where
                $t: std::ops::Div<R, Output = U> + Clone,
                R: Clone,
            {
                type Output = $crate::matrix::Matrix<U>;

                fn div(self, rhs: $crate::matrix::Matrix<R>) -> Self::Output {
                    rhs.scalar_operation_consume_self(&self, |element, scalar| scalar.clone() / element.clone())
                }
            }

            impl<L> std::ops::DivAssign<&$t> for $crate::matrix::Matrix<L>
            where
                L: std::ops::DivAssign<$t>,
                $t: Clone,
            {
                fn div_assign(&mut self, rhs: &$t) {
                    self.scalar_operation_assign(rhs, |element, scalar| *element /= scalar.clone());
                }
            }

            impl<L> std::ops::DivAssign<$t> for $crate::matrix::Matrix<L>
            where
                L: std::ops::DivAssign<$t>,
                $t: Clone,
            {
                fn div_assign(&mut self, rhs: $t) {
                    self.scalar_operation_assign(&rhs, |element, scalar| *element /= scalar.clone());
                }
            }
        )*
    }
}

impl_scalar_div!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

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

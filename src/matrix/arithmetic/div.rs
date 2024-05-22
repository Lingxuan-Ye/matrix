use super::super::Matrix;
use std::ops::{Div, DivAssign};

macro_rules! impl_scalar_div {
    ($($t:ty)*) => {
        $(
            impl Div<&$t> for &Matrix<&$t> {
                type Output = Matrix<$t>;

                fn div(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation(rhs, |x, y| (*x).clone() / y.clone())
                }
            }

            impl Div<$t> for &Matrix<&$t> {
                type Output = Matrix<$t>;

                fn div(self, rhs: $t) -> Self::Output {
                    self.scalar_operation(&rhs, |x, y| (*x).clone() / y.clone())
                }
            }

            impl Div<&$t> for Matrix<&$t> {
                type Output = Matrix<$t>;

                fn div(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation_consume_self(rhs, |x, y| x.clone() / y.clone())
                }
            }

            impl Div<$t> for Matrix<&$t> {
                type Output = Matrix<$t>;

                fn div(self, rhs: $t) -> Self::Output {
                    self.scalar_operation_consume_self(&rhs, |x, y| x.clone() / y.clone())
                }
            }

            impl Div<&$t> for &Matrix<$t> {
                type Output = Matrix<$t>;

                fn div(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation(rhs, |x, y| x.clone() / y.clone())
                }
            }

            impl Div<$t> for &Matrix<$t> {
                type Output = Matrix<$t>;

                fn div(self, rhs: $t) -> Self::Output {
                    self.scalar_operation(&rhs, |x, y| x.clone() / y.clone())
                }
            }

            impl Div<&$t> for Matrix<$t> {
                type Output = Matrix<$t>;

                fn div(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation_consume_self(rhs, |x, y| x / y.clone())
                }
            }

            impl Div<$t> for Matrix<$t> {
                type Output = Matrix<$t>;

                fn div(self, rhs: $t) -> Self::Output {
                    self.scalar_operation_consume_self(&rhs, |x, y| x / y.clone())
                }
            }

            impl Div<&Matrix<&$t>> for &$t {
                type Output = Matrix<$t>;

                fn div(self, rhs: &Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation(self, |x, y| y.clone() / (*x).clone())
                }
            }

            impl Div<Matrix<&$t>> for &$t {
                type Output = Matrix<$t>;

                fn div(self, rhs: Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(self, |x, y| y.clone() / x.clone())
                }
            }

            impl Div<&Matrix<$t>> for &$t {
                type Output = Matrix<$t>;

                fn div(self, rhs: &Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation(self, |x, y| y.clone() / x.clone())
                }
            }

            impl Div<Matrix<$t>> for &$t {
                type Output = Matrix<$t>;

                fn div(self, rhs: Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(self, |x, y| y.clone() / x)
                }
            }

            impl Div<&Matrix<&$t>> for $t {
                type Output = Matrix<$t>;

                fn div(self, rhs: &Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation(&self, |x, y| y.clone() / (*x).clone())
                }
            }

            impl Div<Matrix<&$t>> for $t {
                type Output = Matrix<$t>;

                fn div(self, rhs: Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(&self, |x, y| y.clone() / x.clone())
                }
            }

            impl Div<&Matrix<$t>> for $t {
                type Output = Matrix<$t>;

                fn div(self, rhs: &Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation(&self, |x, y| y.clone() / x.clone())
                }
            }

            impl Div<Matrix<$t>> for $t {
                type Output = Matrix<$t>;

                fn div(self, rhs: Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(&self, |x, y| y.clone() / x)
                }
            }

            impl DivAssign<&$t> for Matrix<$t> {
                fn div_assign(&mut self, rhs: &$t) {
                    self.scalar_operation_assign(rhs, |x, y| *x /= y.clone());
                }
            }

            impl DivAssign<$t> for Matrix<$t> {
                fn div_assign(&mut self, rhs: $t) {
                    self.scalar_operation_assign(&rhs, |x, y| *x /= y.clone());
                }
            }
        )*
    }
}

impl_scalar_div!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn test_matrix_div_scalar() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = 2;
        let expected = matrix![[0, 0, 1], [1, 2, 2]];

        let result = &lhs / rhs;
        assert_eq!(result, expected);

        lhs.switch_order();
        let mut result: Matrix<i32> = &lhs / rhs;
        assert_ne!(result, expected);
        result.switch_order();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_div_matrix() {
        let lhs = 12;
        let mut rhs = matrix![[1, 2, 3], [4, 5, 6]];
        let expected = matrix![[12, 6, 4], [3, 2, 2]];

        let result = lhs / &rhs;
        assert_eq!(result, expected);

        rhs.switch_order();
        let mut result: Matrix<i32> = lhs / &rhs;
        assert_ne!(result, expected);
        result.switch_order();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_div_scalar_consume_matrix() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = 2;
        let expected = matrix![[0, 0, 1], [1, 2, 2]];

        let result = lhs.clone() / rhs;
        assert_eq!(result, expected);

        lhs.switch_order();
        let mut result: Matrix<i32> = lhs.clone() / rhs;
        assert_ne!(result, expected);
        result.switch_order();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_div_matrix_consume_matrix() {
        let lhs = 12;
        let mut rhs = matrix![[1, 2, 3], [4, 5, 6]];
        let expected = matrix![[12, 6, 4], [3, 2, 2]];

        let result = lhs / rhs.clone();
        assert_eq!(result, expected);

        rhs.switch_order();
        let mut result: Matrix<i32> = lhs / rhs.clone();
        assert_ne!(result, expected);
        result.switch_order();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_div_scalar_assign() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = 2;
        let expected = matrix![[0, 0, 1], [1, 2, 2]];

        let mut result = lhs.clone();
        result /= rhs;
        assert_eq!(result, expected);

        lhs.switch_order();
        let mut result = lhs.clone();
        result /= rhs;
        assert_ne!(result, expected);
        result.switch_order();
        assert_eq!(result, expected);
    }
}

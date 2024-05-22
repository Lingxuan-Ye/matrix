use super::super::Matrix;
use crate::marker::Scalar;
use std::ops::{Div, DivAssign};

impl<L, R, T> Div<R> for &Matrix<L>
where
    L: Div<R, Output = T> + Clone,
    R: Scalar + Clone,
{
    type Output = Matrix<T>;

    fn div(self, rhs: R) -> Self::Output {
        self.scalar_operation(&rhs, |x, y| x.clone() / y.clone())
    }
}

impl<L, R, T> Div<R> for Matrix<L>
where
    L: Div<R, Output = T>,
    R: Scalar + Clone,
{
    type Output = Matrix<T>;

    fn div(self, rhs: R) -> Self::Output {
        self.scalar_operation_consume_self(&rhs, |x, y| x / y.clone())
    }
}

impl<L, R> DivAssign<R> for Matrix<L>
where
    L: DivAssign<R>,
    R: Scalar + Clone,
{
    fn div_assign(&mut self, rhs: R) {
        self.scalar_operation_assign(&rhs, |x, y| *x /= y.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    #[test]
    fn test_scalar_div() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = 2;
        let expected = matrix![[0, 0, 1], [1, 2, 2]];

        let result = &lhs / rhs;
        assert_eq!(result, expected);

        lhs.switch_order();
        let mut result = &lhs / rhs;
        assert_ne!(result, expected);
        result.switch_order();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_div_consume_matrix() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = 2;
        let expected = matrix![[0, 0, 1], [1, 2, 2]];

        let result = lhs.clone() / rhs;
        assert_eq!(result, expected);

        lhs.switch_order();
        let mut result = lhs.clone() / rhs;
        assert_ne!(result, expected);
        result.switch_order();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_div_assign() {
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

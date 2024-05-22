use super::super::arithmetic::vector_dot_product;
use super::super::Matrix;
use crate::marker::Scalar;
use std::ops::{Add, Mul, MulAssign};

impl<L, R, T> Mul<&Matrix<R>> for &Matrix<L>
where
    L: Mul<R, Output = T> + Clone,
    R: Clone,
    T: Add<Output = T> + Default,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<R>) -> Self::Output {
        let result = self.multiplication_like_operation(rhs, vector_dot_product);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, T> Mul<Matrix<R>> for &Matrix<L>
where
    L: Mul<R, Output = T> + Clone,
    R: Clone,
    T: Add<Output = T> + Default,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<R>) -> Self::Output {
        let result = self.multiplication_like_operation(&rhs, vector_dot_product);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, T> Mul<&Matrix<R>> for Matrix<L>
where
    L: Mul<R, Output = T> + Clone,
    R: Clone,
    T: Add<Output = T> + Default,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<R>) -> Self::Output {
        let result = self.multiplication_like_operation(rhs, vector_dot_product);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, T> Mul<Matrix<R>> for Matrix<L>
where
    L: Mul<R, Output = T> + Clone,
    R: Clone,
    T: Add<Output = T> + Default,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<R>) -> Self::Output {
        let result = self.multiplication_like_operation(&rhs, vector_dot_product);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, T> Mul<R> for &Matrix<L>
where
    L: Mul<R, Output = T> + Clone,
    R: Scalar + Clone,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: R) -> Self::Output {
        self.scalar_operation(&rhs, |x, y| x.clone() * y.clone())
    }
}

impl<L, R, T> Mul<R> for Matrix<L>
where
    L: Mul<R, Output = T>,
    R: Scalar + Clone,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: R) -> Self::Output {
        self.scalar_operation_consume_self(&rhs, |x, y| x * y.clone())
    }
}

impl<L, R> MulAssign<R> for Matrix<L>
where
    L: MulAssign<R>,
    R: Scalar + Clone,
{
    fn mul_assign(&mut self, rhs: R) {
        self.scalar_operation_assign(&rhs, |x, y| *x *= y.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    #[test]
    fn test_mul() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4], [3, 2], [1, 0]];
        let expected = matrix![[5, 2], [32, 20]];

        let result = &lhs * &rhs;
        assert_eq!(result, expected);

        rhs.switch_order();
        let result = &lhs * &rhs;
        assert_eq!(result, expected);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        rhs.switch_order();

        lhs.switch_order();
        let mut result = &lhs * &rhs;
        assert_ne!(result, expected);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        result.switch_order();
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_mul_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[0, 1, 2], [3, 4, 5]];

        let _ = &lhs * &rhs;
    }

    #[test]
    fn test_mul_consume_rhs() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4], [3, 2], [1, 0]];
        let expected = matrix![[5, 2], [32, 20]];

        let result = &lhs * rhs.clone();
        assert_eq!(result, expected);

        rhs.switch_order();
        let result = &lhs * rhs.clone();
        assert_eq!(result, expected);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        rhs.switch_order();

        lhs.switch_order();
        let mut result = &lhs * rhs.clone();
        assert_ne!(result, expected);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        result.switch_order();
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_mul_consume_rhs_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[0, 1, 2], [3, 4, 5]];

        let _ = &lhs * rhs;
    }

    #[test]
    fn test_mul_consume_lhs() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4], [3, 2], [1, 0]];
        let expected = matrix![[5, 2], [32, 20]];

        let result = lhs.clone() * &rhs;
        assert_eq!(result, expected);

        rhs.switch_order();
        let result = lhs.clone() * &rhs;
        assert_eq!(result, expected);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        rhs.switch_order();

        lhs.switch_order();
        let mut result = lhs.clone() * &rhs;
        assert_ne!(result, expected);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        result.switch_order();
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_mul_consume_lhs_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[0, 1, 2], [3, 4, 5]];

        let _ = lhs * &rhs;
    }

    #[test]
    fn test_mul_consume_both() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4], [3, 2], [1, 0]];
        let expected = matrix![[5, 2], [32, 20]];

        let result = lhs.clone() * rhs.clone();
        assert_eq!(result, expected);

        rhs.switch_order();
        let result = lhs.clone() * rhs.clone();
        assert_eq!(result, expected);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        rhs.switch_order();

        lhs.switch_order();
        let mut result = lhs.clone() * rhs.clone();
        assert_ne!(result, expected);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        result.switch_order();
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_mul_consume_both_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[0, 1, 2], [3, 4, 5]];

        let _ = lhs * rhs;
    }

    #[test]
    fn test_scalar_mul() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = 2;
        let expected = matrix![[0, 2, 4], [6, 8, 10]];

        let result = &lhs * rhs;
        assert_eq!(result, expected);

        lhs.switch_order();
        let mut result = &lhs * rhs;
        assert_ne!(result, expected);
        result.switch_order();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_mul_consume_matrix() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = 2;
        let expected = matrix![[0, 2, 4], [6, 8, 10]];

        let result = lhs.clone() * rhs;
        assert_eq!(result, expected);

        lhs.switch_order();
        let mut result = lhs.clone() * rhs;
        assert_ne!(result, expected);
        result.switch_order();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_mul_assign() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = 2;
        let expected = matrix![[0, 2, 4], [6, 8, 10]];

        let mut result = lhs.clone();
        result *= rhs;
        assert_eq!(result, expected);

        lhs.switch_order();
        let mut result = lhs.clone();
        result *= rhs;
        assert_ne!(result, expected);
        result.switch_order();
        assert_eq!(result, expected);
    }
}

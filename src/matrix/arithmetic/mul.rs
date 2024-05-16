use super::super::operation;
use super::super::Matrix;

impl<L, R, T> std::ops::Mul<&Matrix<R>> for &Matrix<L>
where
    L: std::ops::Mul<R, Output = T> + Clone,
    R: Clone,
    T: std::ops::Add<Output = T> + Default,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<R>) -> Self::Output {
        let result =
            operation::multiplication_like_operation(self, rhs, operation::vector_dot_product);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, T> std::ops::Mul<Matrix<R>> for &Matrix<L>
where
    L: std::ops::Mul<R, Output = T> + Clone,
    R: Clone,
    T: std::ops::Add<Output = T> + Default,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<R>) -> Self::Output {
        let result =
            operation::multiplication_like_operation(self, &rhs, operation::vector_dot_product);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, T> std::ops::Mul<&Matrix<R>> for Matrix<L>
where
    L: std::ops::Mul<R, Output = T> + Clone,
    R: Clone,
    T: std::ops::Add<Output = T> + Default,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<R>) -> Self::Output {
        let result =
            operation::multiplication_like_operation(&self, rhs, operation::vector_dot_product);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, T> std::ops::Mul<Matrix<R>> for Matrix<L>
where
    L: std::ops::Mul<R, Output = T> + Clone,
    R: Clone,
    T: std::ops::Add<Output = T> + Default,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<R>) -> Self::Output {
        let result =
            operation::multiplication_like_operation(&self, &rhs, operation::vector_dot_product);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    #[test]
    fn test_mul() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4], [3, 2], [1, 0]];
        let target = matrix![[5, 2], [32, 20]];

        let result = &lhs * &rhs;
        assert_eq!(result, target);

        rhs.switch_order();
        let result = &lhs * &rhs;
        assert_eq!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        rhs.switch_order();

        lhs.switch_order();
        let mut result = &lhs * &rhs;
        assert_ne!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        result.switch_order();
        assert_eq!(result, target);
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
        let target = matrix![[5, 2], [32, 20]];

        let result = &lhs * rhs.clone();
        assert_eq!(result, target);

        rhs.switch_order();
        let result = &lhs * rhs.clone();
        assert_eq!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        rhs.switch_order();

        lhs.switch_order();
        let mut result = &lhs * rhs.clone();
        assert_ne!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        result.switch_order();
        assert_eq!(result, target);
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
        let target = matrix![[5, 2], [32, 20]];

        let result = lhs.clone() * &rhs;
        assert_eq!(result, target);

        rhs.switch_order();
        let result = lhs.clone() * &rhs;
        assert_eq!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        rhs.switch_order();

        lhs.switch_order();
        let mut result = lhs.clone() * &rhs;
        assert_ne!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        result.switch_order();
        assert_eq!(result, target);
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
        let target = matrix![[5, 2], [32, 20]];

        let result = lhs.clone() * rhs.clone();
        assert_eq!(result, target);

        rhs.switch_order();
        let result = lhs.clone() * rhs.clone();
        assert_eq!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        rhs.switch_order();

        lhs.switch_order();
        let mut result = lhs.clone() * rhs.clone();
        assert_ne!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        result.switch_order();
        assert_eq!(result, target);
    }

    #[test]
    #[should_panic]
    fn test_mul_consume_both_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[0, 1, 2], [3, 4, 5]];

        let _ = lhs * rhs;
    }
}

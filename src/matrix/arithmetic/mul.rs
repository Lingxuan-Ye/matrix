use super::super::Matrix;
use super::vector_dot_product;

impl<L, R, T> std::ops::Mul<&Matrix<R>> for &Matrix<L>
where
    L: std::ops::Mul<R, Output = T> + Clone,
    R: Clone,
    T: std::ops::Add<Output = T> + Default,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<R>) -> Self::Output {
        let result = super::multiplication_like_operation(self, rhs, vector_dot_product);
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
        let result = super::multiplication_like_operation(self, &rhs, vector_dot_product);
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
        let result = super::multiplication_like_operation(&self, rhs, vector_dot_product);
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
        let result = super::multiplication_like_operation(&self, &rhs, vector_dot_product);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::matrix;

    #[test]
    fn test_sub() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[0, 1], [2, 3], [4, 5]];
        let target = matrix![[10, 13], [28, 40]];

        assert_eq!(&lhs * &rhs, target);
        assert_eq!(&lhs * rhs.clone(), target);
        assert_eq!(lhs.clone() * &rhs, target);
        assert_eq!(lhs.clone() * rhs.clone(), target);

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
        result.switch_order();
        assert_eq!(result, target);
        lhs.switch_order();
    }

    #[test]
    #[should_panic]
    fn test_sub_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[0, 1], [2, 3]];

        let _ = lhs * rhs;
    }
}

use super::super::operation;
use super::super::Matrix;

impl<L, R, T> std::ops::Add<&Matrix<R>> for &Matrix<L>
where
    L: std::ops::Add<R, Output = T> + Clone,
    R: Clone,
{
    type Output = Matrix<T>;

    fn add(self, rhs: &Matrix<R>) -> Self::Output {
        let result = operation::elementwise_operation(self, rhs, |(x, y)| x.clone() + y.clone());
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, T> std::ops::Add<Matrix<R>> for &Matrix<L>
where
    L: std::ops::Add<R, Output = T> + Clone,
    R: Clone,
{
    type Output = Matrix<T>;

    fn add(self, rhs: Matrix<R>) -> Self::Output {
        let result =
            operation::elementwise_operation_consume_rhs(self, rhs, |(x, y)| x.clone() + y);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, T> std::ops::Add<&Matrix<R>> for Matrix<L>
where
    L: std::ops::Add<R, Output = T>,
    R: Clone,
{
    type Output = Matrix<T>;

    fn add(self, rhs: &Matrix<R>) -> Self::Output {
        let result =
            operation::elementwise_operation_consume_lhs(self, rhs, |(x, y)| x + y.clone());
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, T> std::ops::Add<Matrix<R>> for Matrix<L>
where
    L: std::ops::Add<R, Output = T>,
    R: Clone,
{
    type Output = Matrix<T>;

    fn add(self, rhs: Matrix<R>) -> Self::Output {
        let result = operation::elementwise_operation_consume_both(self, rhs, |(x, y)| x + y);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R> std::ops::AddAssign<&Matrix<R>> for Matrix<L>
where
    L: std::ops::AddAssign<R>,
    R: Clone,
{
    fn add_assign(&mut self, rhs: &Matrix<R>) {
        let result =
            operation::elementwise_operation_assign_to_lhs(self, rhs, |(x, y)| *x += y.clone());
        if let Err(error) = result {
            panic!("{error}");
        }
    }
}

impl<L, R> std::ops::AddAssign<Matrix<R>> for Matrix<L>
where
    L: std::ops::AddAssign<R>,
    R: Clone,
{
    fn add_assign(&mut self, rhs: Matrix<R>) {
        let result =
            operation::elementwise_operation_assign_to_lhs_consume_rhs(self, rhs, |(x, y)| *x += y);
        if let Err(error) = result {
            panic!("{error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    #[test]
    fn test_add() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4, 3], [2, 1, 0]];
        let target = matrix![[5, 5, 5], [5, 5, 5]];

        let result = &lhs + &rhs;
        assert_eq!(result, target);

        rhs.switch_order();
        let result = &lhs + &rhs;
        assert_eq!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        rhs.switch_order();

        lhs.switch_order();
        let mut result = &lhs + &rhs;
        assert_ne!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        result.switch_order();
        assert_eq!(result, target);
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
        let target = matrix![[5, 5, 5], [5, 5, 5]];

        let result = &lhs + rhs.clone();
        assert_eq!(result, target);

        rhs.switch_order();
        let result = &lhs + rhs.clone();
        assert_eq!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        rhs.switch_order();

        lhs.switch_order();
        let mut result = &lhs + rhs.clone();
        assert_ne!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        result.switch_order();
        assert_eq!(result, target);
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
        let target = matrix![[5, 5, 5], [5, 5, 5]];

        let result = lhs.clone() + &rhs;
        assert_eq!(result, target);

        rhs.switch_order();
        let result = lhs.clone() + &rhs;
        assert_eq!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        rhs.switch_order();

        lhs.switch_order();
        let mut result = lhs.clone() + &rhs;
        assert_ne!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        result.switch_order();
        assert_eq!(result, target);
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
        let target = matrix![[5, 5, 5], [5, 5, 5]];

        let result = lhs.clone() + rhs.clone();
        assert_eq!(result, target);

        rhs.switch_order();
        let result = lhs.clone() + rhs.clone();
        assert_eq!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        rhs.switch_order();

        lhs.switch_order();
        let mut result = lhs.clone() + rhs.clone();
        assert_ne!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        result.switch_order();
        assert_eq!(result, target);
    }

    #[test]
    #[should_panic]
    fn test_add_consume_both_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[1, 1], [2, 2], [3, 3]];

        let _ = lhs + rhs;
    }

    #[test]
    fn test_add_assign_to_lhs() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4, 3], [2, 1, 0]];
        let target = matrix![[5, 5, 5], [5, 5, 5]];

        let mut result = lhs.clone();
        result += &rhs;
        assert_eq!(result, target);

        rhs.switch_order();
        let mut result = lhs.clone();
        result += &rhs;
        assert_eq!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        rhs.switch_order();

        lhs.switch_order();
        let mut result = lhs.clone();
        result += &rhs;
        assert_ne!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        result.switch_order();
        assert_eq!(result, target);
    }

    #[test]
    #[should_panic]
    fn test_add_assign_to_lhs_fails() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[1, 1], [2, 2], [3, 3]];

        lhs += &rhs;
    }

    #[test]
    fn test_add_assign_to_lhs_consume_rhs() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4, 3], [2, 1, 0]];
        let target = matrix![[5, 5, 5], [5, 5, 5]];

        let mut result = lhs.clone();
        result += rhs.clone();
        assert_eq!(result, target);

        rhs.switch_order();
        let mut result = lhs.clone();
        result += rhs.clone();
        assert_eq!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        rhs.switch_order();

        lhs.switch_order();
        let mut result = lhs.clone();
        result += rhs.clone();
        assert_ne!(result, target);
        assert_eq!(result.order, lhs.order);
        assert_ne!(result.order, rhs.order);
        result.switch_order();
        assert_eq!(result, target);
    }

    #[test]
    #[should_panic]
    fn test_add_assign_to_lhs_consume_rhs_fails() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[1, 1], [2, 2], [3, 3]];

        lhs += rhs;
    }
}

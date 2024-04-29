use super::super::Matrix;

impl<L, R, T> std::ops::Add<&Matrix<R>> for &Matrix<L>
where
    L: std::ops::Add<R, Output = T> + Clone,
    R: Clone,
{
    type Output = Matrix<T>;

    fn add(self, rhs: &Matrix<R>) -> Self::Output {
        match self.elementwise_operation(rhs, |(x, y)| x.clone() + y.clone()) {
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
        match self.elementwise_operation_consume_rhs(rhs, |(x, y)| x.clone() + y) {
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
        match self.elementwise_operation_consume_self(rhs, |(x, y)| x + y.clone()) {
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
        match self.elementwise_operation_consume_both(rhs, |(x, y)| x + y) {
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
        match self.elementwise_operation_assign(rhs, |(x, y)| *x += y.clone()) {
            Err(error) => panic!("{error}"),
            _ => (),
        }
    }
}

impl<L, R> std::ops::AddAssign<Matrix<R>> for Matrix<L>
where
    L: std::ops::AddAssign<R>,
    R: Clone,
{
    fn add_assign(&mut self, rhs: Matrix<R>) {
        match self.elementwise_operation_assign_consume_rhs(rhs, |(x, y)| *x += y) {
            Err(error) => panic!("{error}"),
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::matrix;

    #[test]
    fn test_add() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[1, 1, 1], [2, 2, 2]];
        let target = matrix![[1, 2, 3], [5, 6, 7]];

        assert_eq!(&lhs + &rhs, target);
        assert_eq!(&lhs + rhs.clone(), target);
        assert_eq!(lhs.clone() + &rhs, target);
        assert_eq!(lhs.clone() + rhs.clone(), target);

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
        result.switch_order();
        assert_eq!(result, target);
        lhs.switch_order();

        let mut lhs_clone = lhs.clone();
        lhs_clone += &rhs;
        assert_eq!(lhs_clone, target);

        let mut lhs_clone = lhs.clone();
        lhs_clone += rhs;
        assert_eq!(lhs_clone, target);
    }

    #[test]
    #[should_panic]
    fn test_add_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[1, 1], [2, 2], [3, 3]];

        let _ = lhs + rhs;
    }
}
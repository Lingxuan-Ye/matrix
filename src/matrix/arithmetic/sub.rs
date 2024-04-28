use super::super::Matrix;

impl<L, R, T> std::ops::Sub<&Matrix<R>> for &Matrix<L>
where
    L: std::ops::Sub<R, Output = T> + Clone,
    R: Clone,
{
    type Output = Matrix<T>;

    fn sub(self, rhs: &Matrix<R>) -> Self::Output {
        match self.elementwise_operation(rhs, |(x, y)| x.clone() - y.clone()) {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, T> std::ops::Sub<Matrix<R>> for &Matrix<L>
where
    L: std::ops::Sub<R, Output = T> + Clone,
    R: Clone,
{
    type Output = Matrix<T>;

    fn sub(self, rhs: Matrix<R>) -> Self::Output {
        match self.elementwise_operation_consume_rhs(rhs, |(x, y)| x.clone() - y) {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, T> std::ops::Sub<&Matrix<R>> for Matrix<L>
where
    L: std::ops::Sub<R, Output = T>,
    R: Clone,
{
    type Output = Matrix<T>;

    fn sub(self, rhs: &Matrix<R>) -> Self::Output {
        match self.elementwise_operation_consume_self(rhs, |(x, y)| x - y.clone()) {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, T> std::ops::Sub<Matrix<R>> for Matrix<L>
where
    L: std::ops::Sub<R, Output = T>,
    R: Clone,
{
    type Output = Matrix<T>;

    fn sub(self, rhs: Matrix<R>) -> Self::Output {
        match self.elementwise_operation_consume_both(rhs, |(x, y)| x - y) {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R> std::ops::SubAssign<&Matrix<R>> for Matrix<L>
where
    L: std::ops::SubAssign<R>,
    R: Clone,
{
    fn sub_assign(&mut self, rhs: &Matrix<R>) {
        match self.elementwise_operation_assign(rhs, |(x, y)| *x -= y.clone()) {
            Err(error) => panic!("{error}"),
            _ => (),
        }
    }
}

impl<L, R> std::ops::SubAssign<Matrix<R>> for Matrix<L>
where
    L: std::ops::SubAssign<R>,
    R: Clone,
{
    fn sub_assign(&mut self, rhs: Matrix<R>) {
        match self.elementwise_operation_assign_consume_rhs(rhs, |(x, y)| *x -= y) {
            Err(error) => panic!("{error}"),
            _ => (),
        }
    }
}

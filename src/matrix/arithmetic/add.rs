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

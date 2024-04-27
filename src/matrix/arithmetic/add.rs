use super::super::Matrix;

impl<T, U, V> std::ops::Add<&Matrix<U>> for &Matrix<T>
where
    T: std::ops::Add<U, Output = V> + Clone,
    U: Clone,
{
    type Output = Matrix<V>;

    fn add(self, rhs: &Matrix<U>) -> Self::Output {
        match self.elementwise_operation(rhs, |(x, y)| x.clone() + y.clone()) {
            Err(error) => panic!("{error}"),
            Ok(result) => result,
        }
    }
}

impl<T, U, V> std::ops::Add<Matrix<U>> for &Matrix<T>
where
    T: std::ops::Add<U, Output = V> + Clone,
{
    type Output = Matrix<V>;

    fn add(self, rhs: Matrix<U>) -> Self::Output {
        match self.elementwise_operation_consume_rhs(rhs, |(x, y)| x.clone() + y) {
            Err(error) => panic!("{error}"),
            Ok(result) => result,
        }
    }
}

impl<T, U, V> std::ops::Add<&Matrix<U>> for Matrix<T>
where
    T: std::ops::Add<U, Output = V>,
    U: Clone,
{
    type Output = Matrix<V>;

    fn add(self, rhs: &Matrix<U>) -> Self::Output {
        match self.elementwise_operation_consume_self(rhs, |(x, y)| x + y.clone()) {
            Err(error) => panic!("{error}"),
            Ok(result) => result,
        }
    }
}

impl<T, U, V> std::ops::Add<Matrix<U>> for Matrix<T>
where
    T: std::ops::Add<U, Output = V>,
{
    type Output = Matrix<V>;

    fn add(self, rhs: Matrix<U>) -> Self::Output {
        match self.elementwise_operation_consume_both(rhs, |(x, y)| x + y) {
            Err(error) => panic!("{error}"),
            Ok(result) => result,
        }
    }
}

impl<T, U> std::ops::AddAssign<&Matrix<U>> for Matrix<T>
where
    T: std::ops::AddAssign<U>,
    U: Clone,
{
    fn add_assign(&mut self, rhs: &Matrix<U>) {
        match self.elementwise_operation_assign(rhs, |(x, y)| *x += y.clone()) {
            Err(error) => panic!("{error}"),
            _ => (),
        }
    }
}

impl<T, U> std::ops::AddAssign<Matrix<U>> for Matrix<T>
where
    T: std::ops::AddAssign<U>,
{
    fn add_assign(&mut self, rhs: Matrix<U>) {
        match self.elementwise_operation_assign_consume_rhs(rhs, |(x, y)| *x += y) {
            Err(error) => panic!("{error}"),
            _ => (),
        }
    }
}

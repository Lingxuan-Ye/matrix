use super::super::Matrix;

impl<T, U, V> std::ops::Sub<&Matrix<U>> for &Matrix<T>
where
    T: std::ops::Sub<U, Output = V> + Clone,
    U: Clone,
{
    type Output = Matrix<V>;

    fn sub(self, rhs: &Matrix<U>) -> Self::Output {
        match self.elementwise_operation(rhs, |(x, y)| x.clone() - y.clone()) {
            Err(error) => panic!("{error}"),
            Ok(result) => result,
        }
    }
}

impl<T, U, V> std::ops::Sub<Matrix<U>> for &Matrix<T>
where
    T: std::ops::Sub<U, Output = V> + Clone,
{
    type Output = Matrix<V>;

    fn sub(self, rhs: Matrix<U>) -> Self::Output {
        match self.elementwise_operation_consume_rhs(rhs, |(x, y)| x.clone() - y) {
            Err(error) => panic!("{error}"),
            Ok(result) => result,
        }
    }
}

impl<T, U, V> std::ops::Sub<&Matrix<U>> for Matrix<T>
where
    T: std::ops::Sub<U, Output = V>,
    U: Clone,
{
    type Output = Matrix<V>;

    fn sub(self, rhs: &Matrix<U>) -> Self::Output {
        match self.elementwise_operation_consume_self(rhs, |(x, y)| x - y.clone()) {
            Err(error) => panic!("{error}"),
            Ok(result) => result,
        }
    }
}

impl<T, U, V> std::ops::Sub<Matrix<U>> for Matrix<T>
where
    T: std::ops::Sub<U, Output = V>,
{
    type Output = Matrix<V>;

    fn sub(self, rhs: Matrix<U>) -> Self::Output {
        match self.elementwise_operation_consume_both(rhs, |(x, y)| x - y) {
            Err(error) => panic!("{error}"),
            Ok(result) => result,
        }
    }
}

impl<T, U> std::ops::SubAssign<&Matrix<U>> for Matrix<T>
where
    T: std::ops::SubAssign<U>,
    U: Clone,
{
    fn sub_assign(&mut self, rhs: &Matrix<U>) {
        match self.elementwise_operation_assign(rhs, |(x, y)| *x -= y.clone()) {
            Err(error) => panic!("{error}"),
            _ => (),
        }
    }
}

impl<T, U> std::ops::SubAssign<Matrix<U>> for Matrix<T>
where
    T: std::ops::SubAssign<U>,
{
    fn sub_assign(&mut self, rhs: Matrix<U>) {
        match self.elementwise_operation_assign_consume_rhs(rhs, |(x, y)| *x -= y) {
            Err(error) => panic!("{error}"),
            _ => (),
        }
    }
}

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
        match self.multiplication_like_operation(rhs, vector_dot_product) {
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
        match self.multiplication_like_operation(&rhs, vector_dot_product) {
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
        match self.multiplication_like_operation(rhs, vector_dot_product) {
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
        match self.multiplication_like_operation(&rhs, vector_dot_product) {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

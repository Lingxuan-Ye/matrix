use super::super::super::iter::VectorIter;
use super::super::super::order::Order;
use super::super::super::shape::{IntoAxisShape, Shape};
use super::super::super::Matrix;
use crate::error::Result;
use crate::impl_scalar_mul;
use std::ops::{Add, Mul};

impl<L, R, U> Mul<Matrix<R>> for Matrix<L>
where
    L: Mul<R, Output = U> + Clone,
    R: Clone,
    U: Add<Output = U> + Default,
{
    type Output = Matrix<U>;

    fn mul(self, rhs: Matrix<R>) -> Self::Output {
        let result = self.multiply(&rhs);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, U> Mul<&Matrix<R>> for Matrix<L>
where
    L: Mul<R, Output = U> + Clone,
    R: Clone,
    U: Add<Output = U> + Default,
{
    type Output = Matrix<U>;

    fn mul(self, rhs: &Matrix<R>) -> Self::Output {
        let result = self.multiply(rhs);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, U> Mul<Matrix<R>> for &Matrix<L>
where
    L: Mul<R, Output = U> + Clone,
    R: Clone,
    U: Add<Output = U> + Default,
{
    type Output = Matrix<U>;

    fn mul(self, rhs: Matrix<R>) -> Self::Output {
        let result = self.multiply(&rhs);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl<L, R, U> Mul<&Matrix<R>> for &Matrix<L>
where
    L: Mul<R, Output = U> + Clone,
    R: Clone,
    U: Add<Output = U> + Default,
{
    type Output = Matrix<U>;

    fn mul(self, rhs: &Matrix<R>) -> Self::Output {
        let result = self.multiply(rhs);
        match result {
            Err(error) => panic!("{error}"),
            Ok(output) => output,
        }
    }
}

impl_scalar_mul! {u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64}

/// Performs multiplication on two matrices.
///
/// # Errors
///
/// - [`Error::NotConformable`] if the matrices are not conformable.
///
/// # Notes
///
/// The resulting matrix will always have the same order as `self`.
///
/// # Examples
///
/// ```
/// use matreex::{matrix, VectorIter};
///
/// let lhs = matrix![[0, 1, 2], [3, 4, 5]];
/// let rhs = matrix![[0, 1], [2, 3], [4, 5]];
///
/// let result = lhs.multiply(&rhs);
/// assert_eq!(result, Ok(matrix![[10, 13], [28, 40]]));
/// ```
///
/// [`Error::NotConformable`]: crate::error::Error::NotConformable
impl<L> Matrix<L> {
    pub fn multiply<R, U>(&self, rhs: &Matrix<R>) -> Result<Matrix<U>>
    where
        L: std::ops::Mul<R, Output = U> + Clone,
        R: Clone,
        U: std::ops::Add<Output = U> + Default,
    {
        self.ensure_multiplication_like_operation_conformable(rhs)?;

        let nrows = self.nrows();
        let ncols = rhs.ncols();
        let order = self.order;
        let shape = Shape::new(nrows, ncols).try_into_axis_shape(order)?;
        let size = shape.size();
        let mut data = Vec::with_capacity(size);

        if self.ncols() == 0 {
            data.resize_with(size, U::default);
            return Ok(Matrix { order, shape, data });
        }

        match (self.order, rhs.order) {
            (Order::RowMajor, Order::RowMajor) => {
                for row in 0..nrows {
                    for col in 0..ncols {
                        match dot_product_static(
                            unsafe { self.iter_nth_major_axis_vector_unchecked(row) },
                            rhs.iter_nth_minor_axis_vector_unchecked(col),
                        ) {
                            None => unreachable!(),
                            Some(element) => data.push(element),
                        }
                    }
                }
            }

            // best scenario
            (Order::RowMajor, Order::ColMajor) => {
                for row in 0..nrows {
                    for col in 0..ncols {
                        match dot_product_static(
                            unsafe { self.iter_nth_major_axis_vector_unchecked(row) },
                            unsafe { rhs.iter_nth_major_axis_vector_unchecked(col) },
                        ) {
                            None => unreachable!(),
                            Some(element) => data.push(element),
                        }
                    }
                }
            }

            // worst scenario
            (Order::ColMajor, Order::RowMajor) => {
                for col in 0..ncols {
                    for row in 0..nrows {
                        /*
                        In this scenario, dynamic dispatch proves to be more
                        efficient than static dispatch, which can seem
                        counterintuitive but is indeed true.
                        */
                        match dot_product_dynamic(
                            self.iter_nth_minor_axis_vector_unchecked(row),
                            rhs.iter_nth_minor_axis_vector_unchecked(col),
                        ) {
                            None => unreachable!(),
                            Some(element) => data.push(element),
                        }
                    }
                }
            }

            (Order::ColMajor, Order::ColMajor) => {
                for col in 0..ncols {
                    for row in 0..nrows {
                        match dot_product_static(
                            self.iter_nth_minor_axis_vector_unchecked(row),
                            unsafe { rhs.iter_nth_major_axis_vector_unchecked(col) },
                        ) {
                            None => unreachable!(),
                            Some(element) => data.push(element),
                        }
                    }
                }
            }
        }

        Ok(Matrix { order, shape, data })
    }
}

#[inline]
fn dot_product_static<'a, L, R, U>(
    lhs: impl DoubleEndedIterator<Item = &'a L>,
    rhs: impl DoubleEndedIterator<Item = &'a R>,
) -> Option<U>
where
    L: Mul<R, Output = U> + Clone + 'a,
    R: Clone + 'a,
    U: Add<Output = U>,
{
    lhs.zip(rhs)
        .map(|(left, right)| left.clone() * right.clone())
        .reduce(|accumulator, product| accumulator + product)
}

#[inline]
fn dot_product_dynamic<'a, L, R, U>(
    lhs: impl DoubleEndedIterator<Item = &'a L>,
    rhs: impl DoubleEndedIterator<Item = &'a R>,
) -> Option<U>
where
    L: Mul<R, Output = U> + Clone + 'a,
    R: Clone + 'a,
    U: Add<Output = U>,
{
    let lhs: VectorIter<&L> = Box::new(lhs);
    let rhs: VectorIter<&R> = Box::new(rhs);
    dot_product_static(lhs, rhs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn test_mul() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let mut rhs = matrix![[5, 4], [3, 2], [1, 0]];
        let expected = matrix![[5, 2], [32, 20]];

        {
            let result = &lhs * &rhs;
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let result = &lhs * &rhs;
            assert_eq!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);

            rhs.switch_order();
        }

        {
            lhs.switch_order();

            let mut result = &lhs * &rhs;
            assert_ne!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            result.switch_order();
            assert_eq!(result, expected);
        }
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

        {
            let result = &lhs * rhs.clone();
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let result = &lhs * rhs.clone();
            assert_eq!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);

            rhs.switch_order();
        }

        {
            lhs.switch_order();

            let mut result = &lhs * rhs.clone();
            assert_ne!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            result.switch_order();
            assert_eq!(result, expected);
        }
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

        {
            let result = lhs.clone() * &rhs;
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let result = lhs.clone() * &rhs;
            assert_eq!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);

            rhs.switch_order();
        }

        {
            lhs.switch_order();

            let mut result = lhs.clone() * &rhs;
            assert_ne!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            result.switch_order();
            assert_eq!(result, expected);
        }
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

        {
            let result = lhs.clone() * rhs.clone();
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let result = lhs.clone() * rhs.clone();
            assert_eq!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            rhs.switch_order();
        }

        {
            lhs.switch_order();

            let mut result = lhs.clone() * rhs.clone();
            assert_ne!(result, expected);
            assert_eq!(result.order, lhs.order);
            assert_ne!(result.order, rhs.order);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_mul_consume_both_fails() {
        let lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = matrix![[0, 1, 2], [3, 4, 5]];

        let _ = lhs * rhs;
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn test_matrix_mul_scalar() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = 2;
        let expected = matrix![[0, 2, 4], [6, 8, 10]];

        {
            let result = &lhs * &rhs;
            assert_eq!(result, expected);

            let result = &lhs * rhs;
            assert_eq!(result, expected);

            let result = lhs.clone() * &rhs;
            assert_eq!(result, expected);

            let result = lhs.clone() * rhs;
            assert_eq!(result, expected);
        }

        {
            lhs.switch_order();

            let mut result: Matrix<i32> = &lhs * &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = &lhs * rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = lhs.clone() * &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = lhs.clone() * rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn test_scalar_mul_matrix() {
        let lhs = 2;
        let mut rhs = matrix![[0, 1, 2], [3, 4, 5]];
        let expected = matrix![[0, 2, 4], [6, 8, 10]];

        {
            let result = &lhs * &rhs;
            assert_eq!(result, expected);

            let result = lhs * &rhs;
            assert_eq!(result, expected);

            let result = &lhs * rhs.clone();
            assert_eq!(result, expected);

            let result = lhs * rhs.clone();
            assert_eq!(result, expected);
        }

        {
            rhs.switch_order();

            let mut result: Matrix<i32> = &lhs * &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = lhs * &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = &lhs * rhs.clone();
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result: Matrix<i32> = lhs * rhs.clone();
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_matrix_mul_scalar_assign() {
        let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
        let rhs = 2;
        let expected = matrix![[0, 2, 4], [6, 8, 10]];

        {
            let mut result = lhs.clone();
            result *= &rhs;
            assert_eq!(result, expected);

            let mut result = lhs.clone();
            result *= rhs;
            assert_eq!(result, expected);
        }

        {
            lhs.switch_order();

            let mut result = lhs.clone();
            result *= &rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);

            let mut result = lhs.clone();
            result *= rhs;
            assert_ne!(result, expected);
            result.switch_order();
            assert_eq!(result, expected);
        }
    }
}

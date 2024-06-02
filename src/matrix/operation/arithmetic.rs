mod add;
mod div;
mod mul;
mod sub;

use super::super::iter::VectorIter;

/// Computes the dot product of two vectors.
///
/// # Examples
///
/// ```
/// use matreex::{matrix, Matrix};
/// use matreex::matrix::operation::arithmetic::vector_dot_product;
///
/// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
///
/// let lhs = matrix.iter_nth_row(0).unwrap();
/// let rhs = matrix.iter_nth_row(1).unwrap();
/// assert_eq!(vector_dot_product(lhs, rhs), Some(14));
///
/// let lhs = matrix.iter_nth_row(0).unwrap();
/// let rhs = matrix.iter_nth_col(1).unwrap();
/// assert_eq!(vector_dot_product(lhs, rhs), Some(4));
///
/// let lhs = matrix.iter_nth_row(0).unwrap();
/// let zero_rows_matrix = Matrix::<i32>::new((0, 3));
/// let rhs = zero_rows_matrix.iter_nth_col(1).unwrap();
/// assert!(vector_dot_product(lhs, rhs).is_none());
/// ```
pub fn vector_dot_product<L, R, U>(lhs: VectorIter<&L>, rhs: VectorIter<&R>) -> Option<U>
where
    L: std::ops::Mul<R, Output = U> + Clone,
    R: Clone,
    U: std::ops::Add<Output = U>,
{
    lhs.zip(rhs)
        .map(|(left, right)| left.clone() * right.clone())
        .reduce(|accumulator, product| accumulator + product)
}

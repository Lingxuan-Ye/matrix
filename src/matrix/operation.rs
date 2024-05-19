use super::index::translate_index_between_orders_unchecked;
use super::iter::VectorIter;
use super::order::Order;
use super::shape::AxisShape;
use super::Matrix;
use crate::error::{Error, Result};

/// Ensures that two matrices are conformable for element-wise operations.
///
/// # Errors
///
/// - [`Error::MatricesInconformable`] if the matrices are not conformable.
///
/// # Examples
///
/// ```
/// use matreex::{Error, Matrix};
/// use matreex::matrix::operation::ensure_elementwise_operation_conformable;
///
/// let lhs = Matrix::<u8>::new((2, 3));
///
/// let rhs = Matrix::<u8>::new((2, 3));
/// let result = ensure_elementwise_operation_conformable(&lhs, &rhs);
/// assert!(result.is_ok());
///
/// let rhs = Matrix::<u8>::new((2, 2));
/// let result = ensure_elementwise_operation_conformable(&lhs, &rhs);
/// assert_eq!(result, Err(Error::MatricesInconformable));
/// ```
pub fn ensure_elementwise_operation_conformable<L, R>(
    lhs: &Matrix<L>,
    rhs: &Matrix<R>,
) -> Result<()> {
    if lhs.shape() != rhs.shape() {
        Err(Error::MatricesInconformable)
    } else {
        Ok(())
    }
}

/// Ensures that two matrices are conformable for multiplication-like operation.
///
/// # Errors
///
/// - [`Error::MatricesInconformable`] if the matrices are not conformable.
///
/// # Examples
///
/// ```
/// use matreex::{Error, Matrix};
/// use matreex::matrix::operation::ensure_multiplication_like_operation_conformable;
///
/// let lhs = Matrix::<u8>::new((2, 3));
///
/// let rhs = Matrix::<u8>::new((3, 1));
/// let result = ensure_multiplication_like_operation_conformable(&lhs, &rhs);
/// assert!(result.is_ok());
///
/// let rhs = Matrix::<u8>::new((2, 3));
/// let result = ensure_multiplication_like_operation_conformable(&lhs, &rhs);
/// assert_eq!(result, Err(Error::MatricesInconformable));
/// ```
pub fn ensure_multiplication_like_operation_conformable<L, R>(
    lhs: &Matrix<L>,
    rhs: &Matrix<R>,
) -> Result<()> {
    if lhs.ncols() != rhs.nrows() {
        Err(Error::MatricesInconformable)
    } else {
        Ok(())
    }
}

/// Computes the dot product of two vectors.
///
/// # Examples
///
/// ```
/// use matreex::{matrix, Matrix};
/// use matreex::matrix::operation::vector_dot_product;
///
/// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
///
/// let lhs = matrix.iter_nth_row(0);
/// let rhs = matrix.iter_nth_row(1);
/// assert_eq!(vector_dot_product(lhs, rhs), Some(14));
///
/// let lhs = matrix.iter_nth_row(0);
/// let rhs = matrix.iter_nth_col(1);
/// assert_eq!(vector_dot_product(lhs, rhs), Some(4));
///
/// let lhs = matrix.iter_nth_row(0);
/// let zero_rows_matrix = Matrix::<u8>::new((0, 3));
/// let rhs = zero_rows_matrix.iter_nth_col(1);
/// assert!(vector_dot_product(lhs, rhs).is_none());
/// ```
pub fn vector_dot_product<L, R, T>(lhs: VectorIter<&L>, rhs: VectorIter<&R>) -> Option<T>
where
    L: std::ops::Mul<R, Output = T> + Clone,
    R: Clone,
    T: std::ops::Add<Output = T>,
{
    lhs.zip(rhs)
        .map(|(x, y)| x.clone() * y.clone())
        .reduce(|acc, v| acc + v)
}

/// Performs element-wise operation `op` on two matrices.
///
/// # Errors
///
/// - [`Error::MatricesInconformable`] if the matrices are not conformable.
///
/// # Notes
///
/// The resulting matrix will always have the same order as `lhs`.
///
/// # Examples
///
/// ```
/// use matreex::matrix;
/// use matreex::matrix::operation::elementwise_operation;
///
/// let lhs = matrix![[0, 1, 2], [3, 4, 5]];
/// let rhs = matrix![[1, 1, 1], [1, 1, 1]];
///
/// let result = elementwise_operation(&lhs, &rhs, |(x, y)| x + y);
/// assert_eq!(result, Ok(matrix![[1, 2, 3], [4, 5, 6]]));
/// ```
pub fn elementwise_operation<L, R, T, F>(
    lhs: &Matrix<L>,
    rhs: &Matrix<R>,
    mut op: F,
) -> Result<Matrix<T>>
where
    F: FnMut((&L, &R)) -> T,
{
    ensure_elementwise_operation_conformable(lhs, rhs)?;

    let order = lhs.order;
    let shape = lhs.shape;
    let data = if lhs.order == rhs.order {
        lhs.data.iter().zip(rhs.data.iter()).map(op).collect()
    } else {
        lhs.data
            .iter()
            .enumerate()
            .map(|(index, left)| {
                let index = translate_index_between_orders_unchecked(index, lhs.shape);
                let right = unsafe { rhs.data.get_unchecked(index) };
                op((left, right))
            })
            .collect()
    };

    Ok(Matrix { data, order, shape })
}

/// Performs element-wise operation `op` on two matrices, consuming `rhs`.
///
/// # Errors
///
/// - [`Error::MatricesInconformable`] if the matrices are not conformable.
///
/// # Notes
///
/// The resulting matrix will always have the same order as `lhs`.
///
/// # Examples
///
/// ```
/// use matreex::matrix;
/// use matreex::matrix::operation::elementwise_operation_consume_rhs;
///
/// let lhs = matrix![[0, 1, 2], [3, 4, 5]];
/// let rhs = matrix![[1, 1, 1], [1, 1, 1]];
///
/// let result = elementwise_operation_consume_rhs(&lhs, rhs, |(x, y)| x + y);
/// assert_eq!(result, Ok(matrix![[1, 2, 3], [4, 5, 6]]));
/// ```
pub fn elementwise_operation_consume_rhs<L, R, T, F>(
    lhs: &Matrix<L>,
    rhs: Matrix<R>,
    mut op: F,
) -> Result<Matrix<T>>
where
    R: Clone,
    F: FnMut((&L, R)) -> T,
{
    ensure_elementwise_operation_conformable(lhs, &rhs)?;

    let order = lhs.order;
    let shape = lhs.shape;
    let data = if lhs.order == rhs.order {
        lhs.data.iter().zip(rhs.data).map(op).collect()
    } else {
        lhs.data
            .iter()
            .enumerate()
            .map(|(index, left)| {
                let index = translate_index_between_orders_unchecked(index, lhs.shape);
                let right = unsafe { rhs.data.get_unchecked(index).clone() };
                op((left, right))
            })
            .collect()
    };

    Ok(Matrix { data, order, shape })
}

/// Performs element-wise operation `op` on two matrices, consuming `lhs`.
///
/// # Errors
///
/// - [`Error::MatricesInconformable`] if the matrices are not conformable.
///
/// # Notes
///
/// The resulting matrix will always have the same order as `lhs`.
///
/// # Examples
///
/// ```
/// use matreex::matrix;
/// use matreex::matrix::operation::elementwise_operation_consume_lhs;
///
/// let lhs = matrix![[0, 1, 2], [3, 4, 5]];
/// let rhs = matrix![[1, 1, 1], [1, 1, 1]];
///
/// let result = elementwise_operation_consume_lhs(lhs, &rhs, |(x, y)| x + y);
/// assert_eq!(result, Ok(matrix![[1, 2, 3], [4, 5, 6]]));
/// ```
pub fn elementwise_operation_consume_lhs<L, R, T, F>(
    lhs: Matrix<L>,
    rhs: &Matrix<R>,
    mut op: F,
) -> Result<Matrix<T>>
where
    F: FnMut((L, &R)) -> T,
{
    ensure_elementwise_operation_conformable(&lhs, rhs)?;

    let order = lhs.order;
    let shape = lhs.shape;
    let data = if lhs.order == rhs.order {
        lhs.data.into_iter().zip(rhs.data.iter()).map(op).collect()
    } else {
        lhs.data
            .into_iter()
            .enumerate()
            .map(|(index, left)| {
                let index = translate_index_between_orders_unchecked(index, lhs.shape);
                let right = unsafe { rhs.data.get_unchecked(index) };
                op((left, right))
            })
            .collect()
    };

    Ok(Matrix { data, order, shape })
}

/// Performs element-wise operation `op` on two matrices, consuming both.
///
/// # Errors
///
/// - [`Error::MatricesInconformable`] if the matrices are not conformable.
///
/// # Notes
///
/// The resulting matrix will always have the same order as `lhs`.
///
/// # Examples
///
/// ```
/// use matreex::matrix;
/// use matreex::matrix::operation::elementwise_operation_consume_both;
///
/// let lhs = matrix![[0, 1, 2], [3, 4, 5]];
/// let rhs = matrix![[1, 1, 1], [1, 1, 1]];
///
/// let result = elementwise_operation_consume_both(lhs, rhs, |(x, y)| x + y);
/// assert_eq!(result, Ok(matrix![[1, 2, 3], [4, 5, 6]]));
/// ```
pub fn elementwise_operation_consume_both<L, R, T, F>(
    lhs: Matrix<L>,
    rhs: Matrix<R>,
    mut op: F,
) -> Result<Matrix<T>>
where
    R: Clone,
    F: FnMut((L, R)) -> T,
{
    ensure_elementwise_operation_conformable(&lhs, &rhs)?;

    let order = lhs.order;
    let shape = lhs.shape;
    let data = if lhs.order == rhs.order {
        lhs.data.into_iter().zip(rhs.data).map(op).collect()
    } else {
        lhs.data
            .into_iter()
            .enumerate()
            .map(|(index, left)| {
                let index = translate_index_between_orders_unchecked(index, lhs.shape);
                let right = unsafe { rhs.data.get_unchecked(index).clone() };
                op((left, right))
            })
            .collect()
    };

    Ok(Matrix { data, order, shape })
}

/// Performs element-wise operation `op` on two matrices, assigning the result
/// to `lhs`.
///
/// # Errors
///
/// - [`Error::MatricesInconformable`] if the matrices are not conformable.
///
/// # Notes
///
/// The resulting matrix will always have the same order as `lhs`.
///
/// # Examples
///
/// ```
/// use matreex::matrix;
/// use matreex::matrix::operation::elementwise_operation_assign_to_lhs;
///
/// let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
/// let rhs = matrix![[1, 1, 1], [1, 1, 1]];
///
/// elementwise_operation_assign_to_lhs(&mut lhs, &rhs, |(x, y)| *x += y).unwrap();
/// assert_eq!(lhs, matrix![[1, 2, 3], [4, 5, 6]]);
/// ```
pub fn elementwise_operation_assign_to_lhs<L, R, F>(
    lhs: &mut Matrix<L>,
    rhs: &Matrix<R>,
    mut op: F,
) -> Result<()>
where
    F: FnMut((&mut L, &R)),
{
    ensure_elementwise_operation_conformable(lhs, rhs)?;

    if lhs.order == rhs.order {
        lhs.data.iter_mut().zip(rhs.data.iter()).for_each(op);
    } else {
        lhs.data.iter_mut().enumerate().for_each(|(index, left)| {
            let index = translate_index_between_orders_unchecked(index, lhs.shape);
            let right = unsafe { rhs.data.get_unchecked(index) };
            op((left, right))
        });
    }

    Ok(())
}

/// Performs element-wise operation `op` on two matrices, assigning the result
/// to `lhs` and consuming `rhs`.
///
/// # Errors
///
/// - [`Error::MatricesInconformable`] if the matrices are not conformable.
///
/// # Notes
///
/// The resulting matrix will always have the same order as `lhs`.
///
/// # Examples
///
/// ```
/// use matreex::matrix;
/// use matreex::matrix::operation::elementwise_operation_assign_to_lhs_consume_rhs;
///
/// let mut lhs = matrix![[0, 1, 2], [3, 4, 5]];
/// let rhs = matrix![[1, 1, 1], [1, 1, 1]];
///
/// elementwise_operation_assign_to_lhs_consume_rhs(&mut lhs, rhs, |(x, y)| *x += y).unwrap();
/// assert_eq!(lhs, matrix![[1, 2, 3], [4, 5, 6]]);
/// ```
pub fn elementwise_operation_assign_to_lhs_consume_rhs<L, R, F>(
    lhs: &mut Matrix<L>,
    rhs: Matrix<R>,
    mut op: F,
) -> Result<()>
where
    R: Clone,
    F: FnMut((&mut L, R)),
{
    ensure_elementwise_operation_conformable(lhs, &rhs)?;

    if lhs.order == rhs.order {
        lhs.data.iter_mut().zip(rhs.data).for_each(op);
    } else {
        lhs.data.iter_mut().enumerate().for_each(|(index, left)| {
            let index = translate_index_between_orders_unchecked(index, lhs.shape);
            let right = unsafe { rhs.data.get_unchecked(index).clone() };
            op((left, right))
        });
    }

    Ok(())
}

/// Performs multiplicatio-like operation `op` on two matrices.
///
/// # Errors
///
/// - [`Error::MatricesInconformable`] if the matrices are not conformable.
///
/// # Notes
///
/// The resulting matrix will always have the same order as `lhs`.
///
/// # Examples
///
/// ```
/// use matreex::matrix;
/// use matreex::matrix::operation::{
///     multiplication_like_operation,
///     vector_dot_product,
/// };
///
/// let lhs = matrix![[0, 1, 2], [3, 4, 5]];
/// let rhs = matrix![[0, 1], [2, 3], [4, 5]];
///
/// let result = multiplication_like_operation(&lhs, &rhs, vector_dot_product);
/// assert_eq!(result, Ok(matrix![[10, 13], [28, 40]]));
/// ```
pub fn multiplication_like_operation<L, R, T, F>(
    lhs: &Matrix<L>,
    rhs: &Matrix<R>,
    mut op: F,
) -> Result<Matrix<T>>
where
    T: Default,
    F: FnMut(VectorIter<&L>, VectorIter<&R>) -> Option<T>,
{
    ensure_multiplication_like_operation_conformable(lhs, rhs)?;

    let nrows = lhs.nrows();
    let ncols = rhs.ncols();
    let size = nrows.checked_mul(ncols).ok_or(Error::SizeOverflow)?;

    let order = lhs.order;
    let shape = AxisShape::try_from_shape_with((nrows, ncols), order)?;
    let mut data = Vec::with_capacity(size);
    match order {
        Order::RowMajor => {
            'outer: for row in 0..nrows {
                for col in 0..ncols {
                    match op(lhs.iter_nth_row(row), rhs.iter_nth_col(col)) {
                        None => {
                            data.resize_with(size, T::default);
                            break 'outer;
                        }
                        Some(value) => data.push(value),
                    }
                }
            }
        }
        Order::ColMajor => {
            'outer: for col in 0..ncols {
                for row in 0..nrows {
                    match op(lhs.iter_nth_row(row), rhs.iter_nth_col(col)) {
                        None => {
                            data.resize_with(size, T::default);
                            break 'outer;
                        }
                        Some(value) => data.push(value),
                    }
                }
            }
        }
    }

    Ok(Matrix { data, order, shape })
}

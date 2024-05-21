use super::order::Order;
use super::shape::{IntoAxisShape, Shape};
use super::Matrix;
use crate::error::{Error, Result};
use crate::vector::kind::Kind;
use crate::vector::Vector;

impl<T: Clone> Matrix<T> {
    /// Creates a new [`Matrix`] instance from the given slice.
    ///
    /// # Notes
    ///
    /// The matrix returned will always have `1` row and `src.len()` columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::Matrix;
    ///
    /// let slice = [0, 1, 2, 3, 4, 5];
    /// let matrix = Matrix::from_slice(&slice);
    ///
    /// assert_eq!(matrix.nrows(), 1);
    /// assert_eq!(matrix.ncols(), 6);
    /// ```
    pub fn from_slice(src: &[T]) -> Self {
        Self::from(src)
    }
}

impl<T> Matrix<T> {
    /// Creates a new [`Matrix`] instance from the given 2D array.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::Matrix;
    ///
    /// let array = Box::new([[0, 1, 2], [3, 4, 5]]);
    /// let matrix = Matrix::from_2darray(array);
    /// ```
    pub fn from_2darray<const R: usize, const C: usize>(src: Box<[[T; C]; R]>) -> Self {
        Self::from(src)
    }
}

impl<T> From<Vector<T>> for Matrix<T> {
    fn from(value: Vector<T>) -> Self {
        let (nrows, ncols) = match value.kind() {
            Kind::RowVector => (1, value.len()),
            Kind::ColVector => (value.len(), 1),
        };
        let data = value.into();
        let order = Order::default();
        let shape = Shape::new(nrows, ncols).into_axis_shape_unchecked(order);
        Self { data, order, shape }
    }
}

impl<T: Clone> From<&[T]> for Matrix<T> {
    fn from(value: &[T]) -> Self {
        let data = value.to_vec();
        let order = Order::default();
        let shape = Shape::new(1, value.len()).into_axis_shape_unchecked(order);
        Self { data, order, shape }
    }
}

impl<T, const R: usize, const C: usize> From<Box<[[T; C]; R]>> for Matrix<T> {
    fn from(value: Box<[[T; C]; R]>) -> Self {
        let ptr = Box::leak(value).as_mut_ptr() as *mut T;
        let data = unsafe { Vec::from_raw_parts(ptr, R * C, R * C) };
        let order = Order::default();
        let shape = Shape::new(R, C).into_axis_shape_unchecked(order);
        Self { data, order, shape }
    }
}

impl<T, const R: usize, const C: usize> From<[[T; C]; R]> for Matrix<T> {
    fn from(value: [[T; C]; R]) -> Self {
        Self::from(Box::new(value))
    }
}

impl<T: Clone> TryFrom<&[Vec<T>]> for Matrix<T> {
    type Error = Error;

    fn try_from(value: &[Vec<T>]) -> Result<Self> {
        let order = Order::default();
        let nrows = value.len();
        let ncols = value.first().map_or(0, |row| row.len());
        let shape = Shape::new(nrows, ncols).try_into_axis_shape(order)?;
        let mut data = Vec::with_capacity(shape.size());
        for row in value {
            if row.len() != ncols {
                return Err(Error::LengthInconsistent);
            }
            data.extend_from_slice(row);
        }
        Ok(Self { data, order, shape })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matrix, vector};

    // This test ensures that the `matrix!` macro works as expected.
    #[test]
    fn test_from_2darray() {
        let data = vec![0, 1, 2, 3, 4, 5];
        let order = Order::default();
        let shape = Shape::new(2, 3).into_axis_shape_unchecked(order);
        let expected = Matrix { data, order, shape };

        let array = Box::new([[0, 1, 2], [3, 4, 5]]);
        assert_eq!(Matrix::from(array.clone()), expected);
        assert_eq!(Matrix::from(*array.clone()), expected);
        assert_eq!(Matrix::from_2darray(array), expected);
        assert_eq!(matrix![[0, 1, 2], [3, 4, 5]], expected);

        let array = Box::new([[0, 1], [2, 3], [4, 5]]);
        assert_ne!(Matrix::from(array.clone()), expected);
        assert_ne!(Matrix::from(*array.clone()), expected);
        assert_ne!(Matrix::from_2darray(array), expected);
        assert_ne!(matrix![[0, 1], [2, 3], [4, 5]], expected);
    }

    #[test]
    fn test_from_vector() {
        let mut vector = vector![0, 1, 2, 3, 4, 5];

        let expected = matrix![[0, 1, 2, 3, 4, 5]];
        assert_eq!(Matrix::from(vector.clone()), expected);

        vector.transpose();

        let expected = matrix![[0], [1], [2], [3], [4], [5]];
        assert_eq!(Matrix::from(vector), expected);
    }

    #[test]
    fn test_from_slice() {
        let expected = matrix![[0, 1, 2, 3, 4, 5]];

        let array = [0, 1, 2, 3, 4, 5];
        assert_eq!(Matrix::from(&array[..]), expected);
        assert_eq!(Matrix::from_slice(&array), expected);

        let array = [0; 6];
        assert_ne!(Matrix::from(&array[..]), expected);
        assert_ne!(Matrix::from_slice(&array), expected);
    }

    #[test]
    fn test_try_from_slice_of_vecs() {
        let expected = matrix![[0, 1, 2], [3, 4, 5]];

        let vecs = [vec![0, 1, 2], vec![3, 4, 5]];
        assert_eq!(Matrix::try_from(&vecs[..]), Ok(expected));

        let vecs = [vec![0, 1, 2], vec![3, 4]];
        assert_eq!(
            Matrix::<i32>::try_from(&vecs[..]),
            Err(Error::LengthInconsistent)
        );
    }
}

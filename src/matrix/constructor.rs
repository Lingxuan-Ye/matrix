use super::order::Order;
use super::shape::{AxisShape, Shape};
use super::Matrix;
use crate::error::Result;

impl<T: Default> Matrix<T> {
    pub fn new<S: Into<Shape>>(shape: S) -> Self {
        match Self::build(shape) {
            Ok(matrix) => matrix,
            Err(error) => panic!("{error}"),
        }
    }

    pub fn build<S: Into<Shape>>(shape: S) -> Result<Self> {
        let order = Order::default();
        let shape = AxisShape::build(shape, order)?;
        let size = Self::check_size(shape.size())?;
        let data = std::iter::repeat_with(T::default).take(size).collect();
        Ok(Self { data, order, shape })
    }
}

impl<T: Clone> Matrix<T> {
    pub fn from_slice(src: &[T]) -> Self {
        let data = src.to_vec();
        let order = Order::default();
        let shape = AxisShape::build((1, src.len()), order).expect("this will never fail");
        Self { data, order, shape }
    }
}

impl<T> Matrix<T> {
    pub fn from_2darray<const R: usize, const C: usize>(src: Box<[[T; C]; R]>) -> Self {
        let ptr = Box::leak(src).as_mut_ptr() as *mut T;
        let data = unsafe { Vec::from_raw_parts(ptr, R * C, R * C) };
        let order = Order::default();
        let shape = AxisShape::build((R, C), order).expect("this will never fail");
        Self { data, order, shape }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::Error;
    use crate::matrix;

    #[test]
    fn test_from_2darray() {
        let data = vec![0, 1, 2, 3, 4, 5];
        let order = Order::default();
        let shape = AxisShape::build((2, 3), order).unwrap();
        let target = Matrix { data, order, shape };

        let array = Box::new([[0, 1, 2], [3, 4, 5]]);
        assert_eq!(Matrix::from_2darray(array), target);

        let array = Box::new([[0, 1], [2, 3], [4, 5]]);
        assert_ne!(Matrix::from_2darray(array), target);
    }

    #[test]
    fn test_new() {
        let target = matrix![[0, 0, 0], [0, 0, 0]];

        assert_eq!(Matrix::new((2, 3)), target);
        assert_ne!(Matrix::new((3, 2)), target);
    }

    #[test]
    fn test_build() {
        let target = matrix![[0, 0, 0], [0, 0, 0]];

        assert_eq!(Matrix::build((2, 3)).unwrap(), target);
        assert_ne!(Matrix::build((3, 2)).unwrap(), target);

        assert_eq!(
            Matrix::<u8>::build((usize::MAX, 2)),
            Err(Error::SizeOverflow)
        );
    }

    #[test]
    fn test_from_slice() {
        let target = matrix![[0, 1, 2, 3, 4, 5]];

        let slice = [0, 1, 2, 3, 4, 5];
        assert_eq!(Matrix::from_slice(&slice), target);

        let slice = [0; 6];
        assert_ne!(Matrix::from_slice(&slice), target);
    }
}

use super::dimension::Dimension;
use super::Matrix;
use crate::error::Result;
use crate::layout::MemoryLayout;
use crate::shape::{Shape, TryIntoShape};

impl<T: Default> Matrix<T> {
    pub fn new<S: TryIntoShape>(shape: S) -> Self {
        match Self::build(shape) {
            Ok(matrix) => matrix,
            Err(error) => panic!("{error}"),
        }
    }

    pub fn build<S: TryIntoShape>(shape: S) -> Result<Self> {
        let shape = shape.try_into_shape()?;
        let size = Self::check_size(&shape)?;
        let data = std::iter::repeat_with(Default::default)
            .take(size)
            .collect();
        let layout = MemoryLayout::default();
        let dimension = Dimension::from_shape(shape, layout);

        Ok(Self {
            data,
            layout,
            dimension,
        })
    }
}

impl<T: Clone> Matrix<T> {
    pub fn from_slice(src: &[T]) -> Self {
        let shape = Shape::build(1, src.len()).expect("this will never fail");
        let data = src.to_vec();
        let layout = MemoryLayout::default();
        let dimension = Dimension::from_shape(shape, layout);

        Self {
            data,
            layout,
            dimension,
        }
    }
}

impl<T> Matrix<T> {
    pub fn from_2darray<const R: usize, const C: usize>(src: Box<[[T; C]; R]>) -> Self {
        let shape = Shape::build(R, C).expect("this will never fail");
        let ptr = Box::leak(src).as_mut_ptr() as *mut T;
        let data = unsafe { Vec::from_raw_parts(ptr, R * C, R * C) };
        let layout = MemoryLayout::default();
        let dimension = Dimension::from_shape(shape, layout);

        Self {
            data,
            layout,
            dimension,
        }
    }
}

#[macro_export]
macro_rules! matrix {
    [ $($x:expr),+ $(,)? ] => {
        $crate::matrix::Matrix::from_2darray(std::boxed::Box::new([$($x,)+]))
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::Error;
    use crate::matrix;

    #[test]
    fn test_from_2darray() {
        let shape = Shape::build(2, 3).unwrap();
        let layout = MemoryLayout::default();
        let target = Matrix {
            data: vec![0, 1, 2, 3, 4, 5],
            layout,
            dimension: Dimension::from_shape(shape, layout),
        };

        let array = Box::new([[0, 1, 2], [3, 4, 5]]);
        let matrix = Matrix::from_2darray(array);
        assert_eq!(matrix, target);

        let array = Box::new([[0, 1], [2, 3], [4, 5]]);
        let matrix = Matrix::from_2darray(array);
        assert_ne!(matrix, target);
    }

    #[test]
    fn test_from_literal() {
        let array = Box::new([[0, 1, 2], [3, 4, 5]]);
        let target = Matrix::from_2darray(array);

        let matrix = matrix![[0, 1, 2], [3, 4, 5]];
        assert_eq!(matrix, target);

        let matrix = matrix![[0, 1], [2, 3], [4, 5]];
        assert_ne!(matrix, target);
    }

    #[test]
    fn test_new() {
        let target = matrix![[0, 0, 0], [0, 0, 0]];

        assert_eq!(Matrix::new(Shape::build(2, 3).unwrap()), target);
        assert_ne!(Matrix::new(Shape::build(3, 2).unwrap()), target);
    }

    #[test]
    fn test_build() {
        let target = matrix![[0, 0, 0], [0, 0, 0]];

        assert_eq!(Matrix::build(Shape::build(2, 3).unwrap()).unwrap(), target);
        assert_ne!(Matrix::build(Shape::build(3, 2).unwrap()).unwrap(), target);
        assert_eq!(Matrix::build((2, 3)).unwrap(), target);
        assert_ne!(Matrix::build((3, 2)).unwrap(), target);
        assert_eq!(Matrix::build([2, 3]).unwrap(), target);
        assert_ne!(Matrix::build([3, 2]).unwrap(), target);

        assert_eq!(
            Matrix::<u8>::build((usize::MAX, 2)).unwrap_err(),
            Error::SizeOverflow
        );
        assert_eq!(
            Matrix::<u8>::build((2, usize::MAX)).unwrap_err(),
            Error::SizeOverflow
        );
        assert_eq!(
            Matrix::<u8>::build((usize::MAX, usize::MAX)).unwrap_err(),
            Error::SizeOverflow
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

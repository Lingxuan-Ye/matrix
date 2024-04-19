mod error;
mod index;
mod macros;
mod shape;

pub use error::{Error, Result};
pub use index::Index;
pub use shape::{Shape, TryIntoShape};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix<T> {
    shape: Shape,
    data: Vec<T>,
}

impl<T: Default> Matrix<T> {
    pub fn new(shape: Shape) -> Self {
        let data = std::iter::repeat_with(Default::default)
            .take(shape.size())
            .collect();
        Self { shape, data }
    }

    pub fn build<S: TryIntoShape>(shape: S) -> Result<Self> {
        let shape = shape.try_into_shape()?;
        Ok(Self::new(shape))
    }
}

impl<T: Clone> Matrix<T> {
    pub fn from_slice(src: &[T]) -> Self {
        let shape = Shape::build(1, src.len()).expect("this will never fail");
        let data = src.to_vec();
        Self { shape, data }
    }
}

impl<T> Matrix<T> {
    pub fn from_2darray<const R: usize, const C: usize>(src: Box<[[T; C]; R]>) -> Self {
        let shape = Shape::build(R, C).expect("this will never fail");
        let ptr = Box::leak(src).as_mut_ptr() as *mut T;
        let data = unsafe { Vec::from_raw_parts(ptr, R * C, R * C) };
        Self { shape, data }
    }
}

impl<T> Matrix<T> {
    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn nrows(&self) -> usize {
        self.shape.nrows()
    }

    pub fn ncols(&self) -> usize {
        self.shape.ncols()
    }

    pub fn size(&self) -> usize {
        self.shape.size()
    }
}

impl<T> Matrix<T> {
    pub fn reshape<S: TryIntoShape>(&mut self, shape: S) -> Result<()> {
        let shape = shape.try_into_shape()?;
        if shape.size() != self.data.len() {
            return Err(Error::SizeMismatch);
        }
        self.shape = shape;
        Ok(())
    }
}

impl<T: Default> Matrix<T> {
    pub fn resize<S: TryIntoShape>(&mut self, shape: S) -> Result<()> {
        let shape = shape.try_into_shape()?;
        self.data.resize_with(shape.size(), Default::default);
        self.shape = shape;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let matrix = Matrix {
            shape: Shape::build(2, 3).unwrap(),
            data: vec![0; 6],
        };

        assert_eq!(Matrix::<usize>::new(Shape::build(2, 3).unwrap()), matrix);
        assert_ne!(Matrix::<usize>::new(Shape::build(3, 2).unwrap()), matrix);
        assert_eq!(Matrix::<usize>::new((2, 3)), matrix);
        assert_ne!(Matrix::<usize>::new((3, 2)), matrix);
        assert_eq!(Matrix::<usize>::new([2, 3]), matrix);
        assert_ne!(Matrix::<usize>::new([3, 2]), matrix);
    }

    #[test]
    #[should_panic]
    fn test_new_size_is_zero() {
        Matrix::<u8>::new((0, 0));
    }

    #[test]
    #[should_panic]
    fn test_new_size_overflows() {
        Matrix::<u8>::new((usize::MAX, 2));
    }

    #[test]
    fn test_build() {
        let matrix = Matrix {
            shape: Shape::build(2, 3).unwrap(),
            data: vec![0; 6],
        };

        assert_eq!(
            Matrix::<u8>::build(Shape::build(2, 3).unwrap()).unwrap(),
            matrix
        );
        assert_ne!(
            Matrix::<u8>::build(Shape::build(3, 2).unwrap()).unwrap(),
            matrix
        );
        assert_eq!(Matrix::<u8>::build((2, 3)).unwrap(), matrix);
        assert_ne!(Matrix::<u8>::build((3, 2)).unwrap(), matrix);
        assert_eq!(Matrix::<u8>::build([2, 3]).unwrap(), matrix);
        assert_ne!(Matrix::<u8>::build([3, 2]).unwrap(), matrix);

        assert_eq!(Matrix::<u8>::build((0, 0)).unwrap_err(), Error::ZeroSize);
        assert_eq!(Matrix::<u8>::build((0, 1)).unwrap_err(), Error::ZeroSize);
        assert_eq!(Matrix::<u8>::build((1, 0)).unwrap_err(), Error::ZeroSize);

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
        let slice = [0, 1, 2, 3, 4, 5];
        let matrix = Matrix {
            shape: Shape::build(1, 6).unwrap(),
            data: slice.to_vec(),
        };

        assert_eq!(Matrix::<u8>::from_slice(&slice), matrix);
    }

    #[test]
    fn test_reshape() {
        let mut matrix = Matrix::<u8>::new((1, 6));

        matrix.reshape((2, 3)).unwrap();
        assert_eq!(matrix.shape, Shape::build(2, 3).unwrap());

        matrix.reshape((3, 2)).unwrap();
        assert_eq!(matrix.shape, Shape::build(3, 2).unwrap());

        assert_eq!(matrix.reshape((0, 1)).unwrap_err(), Error::ZeroSize);

        assert_eq!(
            matrix.reshape((usize::MAX, 2)).unwrap_err(),
            Error::SizeOverflow
        );

        assert_eq!(matrix.reshape((3, 4)).unwrap_err(), Error::SizeMismatch);
    }

    #[test]
    fn test_resize() {
        let slice = [0, 1, 2, 3, 4, 5];
        let mut matrix = Matrix::from_slice(&slice);

        matrix.resize((2, 3)).unwrap();
        assert_eq!(matrix.shape, Shape::build(2, 3).unwrap());
        assert_eq!(matrix.data, slice.to_vec());

        matrix.resize((2, 2)).unwrap();
        assert_eq!(matrix.shape, Shape::build(2, 2).unwrap());
        assert_eq!(matrix.data, vec![0, 1, 2, 3]);

        matrix.resize((3, 3)).unwrap();
        assert_eq!(matrix.shape, Shape::build(3, 3).unwrap());
        assert_eq!(matrix.data, vec![0, 1, 2, 3, 0, 0, 0, 0, 0]);
    }
}

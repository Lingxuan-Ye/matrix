pub mod error;
pub mod shape;

use error::Result;
pub use shape::Shape;
use shape::TryIntoShape;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix<T> {
    shape: Shape,
    data: Vec<T>,
}

impl<T: Clone + Default> Matrix<T> {
    pub fn build<S: TryIntoShape>(shape: S) -> Result<Self> {
        let shape = shape.try_into_shape()?;
        let data = vec![Default::default(); shape.size()];
        Ok(Self { shape, data })
    }

    pub fn new<S: TryIntoShape>(shape: S) -> Self {
        match Self::build(shape) {
            Ok(matrix) => matrix,
            Err(error) => panic!("{error}"),
        }
    }
}

impl<T: Clone> Matrix<T> {
    pub fn from_slice(src: &[T]) -> Self {
        Self {
            shape: Shape::build(1, src.len()).expect("this will never fail"),
            data: src.to_vec(),
        }
    }
}

pub mod error;
pub mod shape;

use error::{Error, Result};
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
        let size = shape.size();
        let datalen = self.data.len();
        if size != datalen {
            return Err(Error::SizeMismatch);
        }
        self.shape = shape;
        Ok(())
    }
}

impl<T: Default> Matrix<T> {
    pub fn resize<S: TryIntoShape>(&mut self, shape: S) -> Result<()> {
        let shape = shape.try_into_shape()?;
        let size = shape.size();
        let datalen = self.data.len();
        if size < datalen {
            self.data.truncate(size);
        } else {
            for _ in datalen..size {
                self.data.push(Default::default())
            }
        }
        self.shape = shape;
        Ok(())
    }
}

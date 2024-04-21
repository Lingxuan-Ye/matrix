use super::Matrix;
use crate::shape::Shape;

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

use super::shape::Shape;
use super::Matrix;
use crate::Order;

impl<T> Matrix<T> {
    pub fn shape(&self) -> Shape {
        self.shape.interpret_with(self.order)
    }

    pub fn nrows(&self) -> usize {
        self.shape.interpret_nrows_with(self.order)
    }

    pub fn ncols(&self) -> usize {
        self.shape.interpret_ncols_with(self.order)
    }

    pub fn size(&self) -> usize {
        self.shape.size()
    }

    pub fn order(&self) -> Order {
        self.order
    }
}

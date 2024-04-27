use super::order::Order;
use super::shape::Shape;
use super::Matrix;

impl<T> Matrix<T> {
    pub fn order(&self) -> Order {
        self.order
    }

    pub fn shape(&self) -> Shape {
        self.shape.interpret_with(self.order)
    }

    pub fn nrows(&self) -> usize {
        self.shape.interpret_nrows_with(self.order)
    }

    pub fn ncols(&self) -> usize {
        self.shape.interpret_ncols_with(self.order)
    }

    pub fn row_stride(&self) -> usize {
        self.ncols()
    }

    pub const fn col_stride(&self) -> usize {
        1
    }

    pub fn size(&self) -> usize {
        self.shape.size()
    }

    pub(crate) fn major(&self) -> usize {
        self.shape.major()
    }

    pub(crate) fn minor(&self) -> usize {
        self.shape.minor()
    }

    pub(crate) fn major_stride(&self) -> usize {
        self.shape.major_stride()
    }

    #[allow(unused)]
    pub(crate) const fn minor_stride(&self) -> usize {
        self.shape.minor_stride()
    }
}

mod count;
mod major_axis;
mod minor_axis;

use super::order::Order;
use super::Matrix;
use major_axis::{MajorAxisMatrixIter, MajorAxisVectorIter};
use minor_axis::{MinorAxisMatrixIter, MinorAxisVectorIter};

pub type VectorIter<'a, T> = Box<dyn Iterator<Item = T> + 'a>;
pub type MatrixIter<'a, T> = Box<dyn DoubleEndedIterator<Item = VectorIter<'a, T>> + 'a>;

impl<T> Matrix<T> {
    pub fn iter_rows(&self) -> MatrixIter<&T> {
        match self.order {
            Order::RowMajor => Box::new(MajorAxisMatrixIter::new(self)),
            Order::ColMajor => Box::new(MinorAxisMatrixIter::new(self)),
        }
    }

    pub fn iter_cols(&self) -> MatrixIter<&T> {
        match self.order {
            Order::RowMajor => Box::new(MinorAxisMatrixIter::new(self)),
            Order::ColMajor => Box::new(MajorAxisMatrixIter::new(self)),
        }
    }
}

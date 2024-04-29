mod count;
mod major_axis;
mod minor_axis;

use self::major_axis::{MajorAxisMatrixIter, MajorAxisVectorIter};
use self::minor_axis::{MinorAxisMatrixIter, MinorAxisVectorIter};
use super::order::Order;
use super::Matrix;

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

    pub fn iter_nth_row(&self, n: usize) -> VectorIter<&T> {
        match self.order {
            Order::RowMajor => Box::new(MajorAxisVectorIter::new(self, n)),
            Order::ColMajor => Box::new(MinorAxisVectorIter::new(self, n)),
        }
    }

    pub fn iter_nth_col(&self, n: usize) -> VectorIter<&T> {
        match self.order {
            Order::RowMajor => Box::new(MinorAxisVectorIter::new(self, n)),
            Order::ColMajor => Box::new(MajorAxisVectorIter::new(self, n)),
        }
    }
}

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
    /// Returns an iterator over the rows of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    /// let mut iter = matrix.iter_rows();
    ///
    /// let row_0: Vec<&u8> = iter.next().unwrap().collect();
    /// assert_eq!(row_0, vec![&0, &1, &2]);
    ///
    /// let row_1: Vec<&u8> = iter.next().unwrap().collect();
    /// assert_eq!(row_1, vec![&3, &4, &5]);
    ///
    /// assert!(iter.next().is_none());
    /// ```
    pub fn iter_rows(&self) -> MatrixIter<&T> {
        match self.order {
            Order::RowMajor => Box::new(MajorAxisMatrixIter::new(self)),
            Order::ColMajor => Box::new(MinorAxisMatrixIter::new(self)),
        }
    }

    /// Returns an iterator over the columns of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    /// let mut iter = matrix.iter_cols();
    ///
    /// let col_0: Vec<&u8> = iter.next().unwrap().collect();
    /// assert_eq!(col_0, vec![&0, &3]);
    ///
    /// let col_1: Vec<&u8> = iter.next().unwrap().collect();
    /// assert_eq!(col_1, vec![&1, &4]);
    ///
    /// let col_2: Vec<&u8> = iter.next().unwrap().collect();
    /// assert_eq!(col_2, vec![&2, &5]);
    ///
    /// assert!(iter.next().is_none());
    /// ```
    pub fn iter_cols(&self) -> MatrixIter<&T> {
        match self.order {
            Order::RowMajor => Box::new(MinorAxisMatrixIter::new(self)),
            Order::ColMajor => Box::new(MajorAxisMatrixIter::new(self)),
        }
    }

    /// Returns an iterator over the elements of the nth row in the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// let mut iter = matrix.iter_nth_row(1);
    /// let row_1: Vec<&u8> = iter.collect();
    /// assert_eq!(row_1, vec![&3, &4, &5]);
    /// ```
    pub fn iter_nth_row(&self, n: usize) -> VectorIter<&T> {
        match self.order {
            Order::RowMajor => Box::new(MajorAxisVectorIter::new(self, n)),
            Order::ColMajor => Box::new(MinorAxisVectorIter::new(self, n)),
        }
    }

    /// Returns an iterator over the elements of the nth column in the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// let mut iter = matrix.iter_nth_col(1);
    /// let col_1: Vec<&u8> = iter.collect();
    /// assert_eq!(col_1, vec![&1, &4]);
    /// ```
    pub fn iter_nth_col(&self, n: usize) -> VectorIter<&T> {
        match self.order {
            Order::RowMajor => Box::new(MinorAxisVectorIter::new(self, n)),
            Order::ColMajor => Box::new(MajorAxisVectorIter::new(self, n)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    #[test]
    fn test_iter_rows() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];

        {
            let mut iter = matrix.iter_rows();
            let row_0: Vec<&u8> = iter.next().unwrap().collect();
            assert_eq!(row_0, vec![&0, &1, &2]);
            let row_1: Vec<&u8> = iter.next().unwrap().collect();
            assert_eq!(row_1, vec![&3, &4, &5]);

            let mut iter = matrix.iter_rows();
            let row_1: Vec<&u8> = iter.next_back().unwrap().collect();
            assert_eq!(row_1, vec![&3, &4, &5]);
            let row_0: Vec<&u8> = iter.next().unwrap().collect();
            assert_eq!(row_0, vec![&0, &1, &2]);
            assert!(iter.next_back().is_none());
            assert!(iter.next().is_none());
        }

        matrix.switch_order();

        {
            let mut iter = matrix.iter_rows();
            let row_0: Vec<&u8> = iter.next().unwrap().collect();
            assert_eq!(row_0, vec![&0, &1, &2]);
            let row_1: Vec<&u8> = iter.next().unwrap().collect();
            assert_eq!(row_1, vec![&3, &4, &5]);

            let mut iter = matrix.iter_rows();
            let row_1: Vec<&u8> = iter.next_back().unwrap().collect();
            assert_eq!(row_1, vec![&3, &4, &5]);
            let row_0: Vec<&u8> = iter.next().unwrap().collect();
            assert_eq!(row_0, vec![&0, &1, &2]);
            assert!(iter.next_back().is_none());
            assert!(iter.next().is_none());
        }
    }

    #[test]
    fn test_iter_cols() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];

        {
            let mut iter = matrix.iter_cols();
            let col_0: Vec<&u8> = iter.next().unwrap().collect();
            assert_eq!(col_0, vec![&0, &3]);
            let col_1: Vec<&u8> = iter.next().unwrap().collect();
            assert_eq!(col_1, vec![&1, &4]);
            let col_2: Vec<&u8> = iter.next().unwrap().collect();
            assert_eq!(col_2, vec![&2, &5]);

            let mut iter = matrix.iter_cols();
            let col_2: Vec<&u8> = iter.next_back().unwrap().collect();
            assert_eq!(col_2, vec![&2, &5]);
            let col_0: Vec<&u8> = iter.next().unwrap().collect();
            assert_eq!(col_0, vec![&0, &3]);
            let col_1: Vec<&u8> = iter.next_back().unwrap().collect();
            assert_eq!(col_1, vec![&1, &4]);
            assert!(iter.next_back().is_none());
            assert!(iter.next().is_none());
        }

        matrix.switch_order();

        {
            let mut iter = matrix.iter_cols();
            let col_0: Vec<&u8> = iter.next().unwrap().collect();
            assert_eq!(col_0, vec![&0, &3]);
            let col_1: Vec<&u8> = iter.next().unwrap().collect();
            assert_eq!(col_1, vec![&1, &4]);
            let col_2: Vec<&u8> = iter.next().unwrap().collect();
            assert_eq!(col_2, vec![&2, &5]);

            let mut iter = matrix.iter_cols();
            let col_2: Vec<&u8> = iter.next_back().unwrap().collect();
            assert_eq!(col_2, vec![&2, &5]);
            let col_0: Vec<&u8> = iter.next().unwrap().collect();
            assert_eq!(col_0, vec![&0, &3]);
            let col_1: Vec<&u8> = iter.next_back().unwrap().collect();
            assert_eq!(col_1, vec![&1, &4]);
            assert!(iter.next_back().is_none());
            assert!(iter.next().is_none());
        }
    }

    #[test]
    fn test_iter_nth_row() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];

        let row_0: Vec<&u8> = matrix.iter_nth_row(0).collect();
        assert_eq!(row_0, vec![&0, &1, &2]);
        let row_1: Vec<&u8> = matrix.iter_nth_row(1).collect();
        assert_eq!(row_1, vec![&3, &4, &5]);

        matrix.switch_order();

        let row_0: Vec<&u8> = matrix.iter_nth_row(0).collect();
        assert_eq!(row_0, vec![&0, &1, &2]);
        let row_1: Vec<&u8> = matrix.iter_nth_row(1).collect();
        assert_eq!(row_1, vec![&3, &4, &5]);
    }

    #[test]
    fn test_iter_nth_col() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];

        let col_0: Vec<&u8> = matrix.iter_nth_col(0).collect();
        assert_eq!(col_0, vec![&0, &3]);
        let col_1: Vec<&u8> = matrix.iter_nth_col(1).collect();
        assert_eq!(col_1, vec![&1, &4]);
        let col_2: Vec<&u8> = matrix.iter_nth_col(2).collect();
        assert_eq!(col_2, vec![&2, &5]);

        matrix.switch_order();

        let col_0: Vec<&u8> = matrix.iter_nth_col(0).collect();
        assert_eq!(col_0, vec![&0, &3]);
        let col_1: Vec<&u8> = matrix.iter_nth_col(1).collect();
        assert_eq!(col_1, vec![&1, &4]);
        let col_2: Vec<&u8> = matrix.iter_nth_col(2).collect();
        assert_eq!(col_2, vec![&2, &5]);
    }
}

use super::order::Order;
use super::Matrix;
use crate::iter::{MatrixIter, VectorIter};
use std::iter::{Skip, StepBy};
use std::ops::Range;
use std::slice::Iter;

impl<T> Matrix<T> {
    /// Returns an iterator over the rows of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    /// let mut rows = matrix.iter_rows();
    ///
    /// let row_0: Vec<&u8> = rows.next().unwrap().collect();
    /// assert_eq!(row_0, vec![&0, &1, &2]);
    ///
    /// let row_1: Vec<&u8> = rows.next().unwrap().collect();
    /// assert_eq!(row_1, vec![&3, &4, &5]);
    ///
    /// assert!(rows.next().is_none());
    /// ```
    pub fn iter_rows(&self) -> MatrixIter<&T> {
        match self.order {
            Order::RowMajor => Box::new(MajorAxisIter::new(self)),
            Order::ColMajor => Box::new(MinorAxisIter::new(self)),
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
    /// let mut cols = matrix.iter_cols();
    ///
    /// let col_0: Vec<&u8> = cols.next().unwrap().collect();
    /// assert_eq!(col_0, vec![&0, &3]);
    ///
    /// let col_1: Vec<&u8> = cols.next().unwrap().collect();
    /// assert_eq!(col_1, vec![&1, &4]);
    ///
    /// let col_2: Vec<&u8> = cols.next().unwrap().collect();
    /// assert_eq!(col_2, vec![&2, &5]);
    ///
    /// assert!(cols.next().is_none());
    /// ```
    pub fn iter_cols(&self) -> MatrixIter<&T> {
        match self.order {
            Order::RowMajor => Box::new(MinorAxisIter::new(self)),
            Order::ColMajor => Box::new(MajorAxisIter::new(self)),
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
    /// let row_1: Vec<&u8> = matrix.iter_nth_row(1).collect();
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
    /// let col_1: Vec<&u8> = matrix.iter_nth_col(1).collect();
    /// assert_eq!(col_1, vec![&1, &4]);
    /// ```
    pub fn iter_nth_col(&self, n: usize) -> VectorIter<&T> {
        match self.order {
            Order::RowMajor => Box::new(MinorAxisVectorIter::new(self, n)),
            Order::ColMajor => Box::new(MajorAxisVectorIter::new(self, n)),
        }
    }
}

#[derive(Clone, Debug)]
struct MajorAxisVectorIter<'a, T>(Iter<'a, T>);

impl<'a, T> MajorAxisVectorIter<'a, T> {
    fn new(matrix: &'a Matrix<T>, nth: usize) -> Self {
        let major_stride = matrix.major_stride();
        let start = nth * major_stride;
        let end = start + major_stride;
        Self(matrix.data[start..end].iter())
    }
}

impl<'a, T> Iterator for MajorAxisVectorIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, T> DoubleEndedIterator for MajorAxisVectorIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

#[derive(Clone, Debug)]
struct MajorAxisIter<'a, T> {
    matrix: &'a Matrix<T>,
    range: Range<usize>,
}

impl<'a, T> MajorAxisIter<'a, T> {
    fn new(matrix: &'a Matrix<T>) -> Self {
        let range = 0..matrix.major();
        Self { matrix, range }
    }
}

impl<'a, T> Iterator for MajorAxisIter<'a, T> {
    type Item = VectorIter<'a, &'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        let nth = self.range.next()?;
        Some(Box::new(MajorAxisVectorIter::new(self.matrix, nth)))
    }
}

impl<'a, T> DoubleEndedIterator for MajorAxisIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let nth = self.range.next_back()?;
        Some(Box::new(MajorAxisVectorIter::new(self.matrix, nth)))
    }
}

#[derive(Clone, Debug)]
struct MinorAxisVectorIter<'a, T>(StepBy<Skip<Iter<'a, T>>>);

impl<'a, T> MinorAxisVectorIter<'a, T> {
    fn new(matrix: &'a Matrix<T>, nth: usize) -> Self {
        Self(matrix.data.iter().skip(nth).step_by(matrix.major_stride()))
    }
}

impl<'a, T> Iterator for MinorAxisVectorIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, T> DoubleEndedIterator for MinorAxisVectorIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

#[derive(Clone, Debug)]
struct MinorAxisIter<'a, T> {
    matrix: &'a Matrix<T>,
    range: Range<usize>,
}

impl<'a, T> MinorAxisIter<'a, T> {
    fn new(matrix: &'a Matrix<T>) -> Self {
        let range = 0..matrix.minor();
        Self { matrix, range }
    }
}

impl<'a, T> Iterator for MinorAxisIter<'a, T> {
    type Item = VectorIter<'a, &'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        let nth = self.range.next()?;
        Some(Box::new(MinorAxisVectorIter::new(self.matrix, nth)))
    }
}

impl<'a, T> DoubleEndedIterator for MinorAxisIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let nth = self.range.next_back()?;
        Some(Box::new(MinorAxisVectorIter::new(self.matrix, nth)))
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    #[test]
    fn test_iter_rows() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];

        {
            let mut rows = matrix.iter_rows();
            let row_0: Vec<&u8> = rows.next().unwrap().collect();
            assert_eq!(row_0, vec![&0, &1, &2]);
            let row_1: Vec<&u8> = rows.next().unwrap().collect();
            assert_eq!(row_1, vec![&3, &4, &5]);

            let mut rows = matrix.iter_rows();
            let row_1: Vec<&u8> = rows.next_back().unwrap().collect();
            assert_eq!(row_1, vec![&3, &4, &5]);
            let row_0: Vec<&u8> = rows.next().unwrap().collect();
            assert_eq!(row_0, vec![&0, &1, &2]);
            assert!(rows.next_back().is_none());
            assert!(rows.next().is_none());
        }

        matrix.switch_order();

        {
            let mut rows = matrix.iter_rows();
            let row_0: Vec<&u8> = rows.next().unwrap().collect();
            assert_eq!(row_0, vec![&0, &1, &2]);
            let row_1: Vec<&u8> = rows.next().unwrap().collect();
            assert_eq!(row_1, vec![&3, &4, &5]);

            let mut rows = matrix.iter_rows();
            let row_1: Vec<&u8> = rows.next_back().unwrap().collect();
            assert_eq!(row_1, vec![&3, &4, &5]);
            let row_0: Vec<&u8> = rows.next().unwrap().collect();
            assert_eq!(row_0, vec![&0, &1, &2]);
            assert!(rows.next_back().is_none());
            assert!(rows.next().is_none());
        }
    }

    #[test]
    fn test_iter_cols() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];

        {
            let mut cols = matrix.iter_cols();
            let col_0: Vec<&u8> = cols.next().unwrap().collect();
            assert_eq!(col_0, vec![&0, &3]);
            let col_1: Vec<&u8> = cols.next().unwrap().collect();
            assert_eq!(col_1, vec![&1, &4]);
            let col_2: Vec<&u8> = cols.next().unwrap().collect();
            assert_eq!(col_2, vec![&2, &5]);

            let mut cols = matrix.iter_cols();
            let col_2: Vec<&u8> = cols.next_back().unwrap().collect();
            assert_eq!(col_2, vec![&2, &5]);
            let col_0: Vec<&u8> = cols.next().unwrap().collect();
            assert_eq!(col_0, vec![&0, &3]);
            let col_1: Vec<&u8> = cols.next_back().unwrap().collect();
            assert_eq!(col_1, vec![&1, &4]);
            assert!(cols.next_back().is_none());
            assert!(cols.next().is_none());
        }

        matrix.switch_order();

        {
            let mut cols = matrix.iter_cols();
            let col_0: Vec<&u8> = cols.next().unwrap().collect();
            assert_eq!(col_0, vec![&0, &3]);
            let col_1: Vec<&u8> = cols.next().unwrap().collect();
            assert_eq!(col_1, vec![&1, &4]);
            let col_2: Vec<&u8> = cols.next().unwrap().collect();
            assert_eq!(col_2, vec![&2, &5]);

            let mut cols = matrix.iter_cols();
            let col_2: Vec<&u8> = cols.next_back().unwrap().collect();
            assert_eq!(col_2, vec![&2, &5]);
            let col_0: Vec<&u8> = cols.next().unwrap().collect();
            assert_eq!(col_0, vec![&0, &3]);
            let col_1: Vec<&u8> = cols.next_back().unwrap().collect();
            assert_eq!(col_1, vec![&1, &4]);
            assert!(cols.next_back().is_none());
            assert!(cols.next().is_none());
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

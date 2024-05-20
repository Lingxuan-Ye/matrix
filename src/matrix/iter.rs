use super::order::Order;
use super::Matrix;
use crate::error::{Error, Result};
use crate::iter::{MatrixIter, VectorIter};

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
    /// let mut row_0 = rows.next().unwrap();
    /// assert_eq!(row_0.next(), Some(&0));
    /// assert_eq!(row_0.next(), Some(&1));
    /// assert_eq!(row_0.next(), Some(&2));
    /// assert_eq!(row_0.next(), None);
    ///
    /// let mut row_1 = rows.next().unwrap();
    /// assert_eq!(row_1.next(), Some(&3));
    /// assert_eq!(row_1.next(), Some(&4));
    /// assert_eq!(row_1.next(), Some(&5));
    /// assert_eq!(row_1.next(), None);
    ///
    /// assert!(rows.next().is_none());
    /// ```
    pub fn iter_rows(&self) -> MatrixIter<&T> {
        match self.order {
            Order::RowMajor => self.iter_by_major_axis(),
            Order::ColMajor => self.iter_by_minor_axis(),
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
    /// let mut col_0 = cols.next().unwrap();
    /// assert_eq!(col_0.next(), Some(&0));
    /// assert_eq!(col_0.next(), Some(&3));
    /// assert_eq!(col_0.next(), None);
    ///
    /// let mut col_1 = cols.next().unwrap();
    /// assert_eq!(col_1.next(), Some(&1));
    /// assert_eq!(col_1.next(), Some(&4));
    /// assert_eq!(col_1.next(), None);
    ///
    /// let mut col_2 = cols.next().unwrap();
    /// assert_eq!(col_2.next(), Some(&2));
    /// assert_eq!(col_2.next(), Some(&5));
    /// assert_eq!(col_2.next(), None);
    ///
    /// assert!(cols.next().is_none());
    /// ```
    pub fn iter_cols(&self) -> MatrixIter<&T> {
        match self.order {
            Order::RowMajor => self.iter_by_minor_axis(),
            Order::ColMajor => self.iter_by_major_axis(),
        }
    }

    /// Returns an iterator over the elements of the nth row in the matrix.
    ///
    /// # Errors
    ///
    /// - [`Error::IndexOutOfBounds`] if `n` is greater than or equal to
    /// the number of rows in the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// let mut row_1 = matrix.iter_nth_row(1).unwrap();
    /// assert_eq!(row_1.next(), Some(&3));
    /// assert_eq!(row_1.next(), Some(&4));
    /// assert_eq!(row_1.next(), Some(&5));
    /// assert_eq!(row_1.next(), None);
    /// ```
    pub fn iter_nth_row(&self, n: usize) -> Result<VectorIter<&T>> {
        match self.order {
            Order::RowMajor => self.iter_nth_major_axis_vector(n),
            Order::ColMajor => self.iter_nth_minor_axis_vector(n),
        }
    }

    /// Returns an iterator over the elements of the nth column in the matrix.
    ///
    /// # Errors
    ///
    /// - [`Error::IndexOutOfBounds`] if `n` is greater than or equal to
    /// the number of columns in the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// let mut col_1 = matrix.iter_nth_col(1).unwrap();
    /// assert_eq!(col_1.next(), Some(&1));
    /// assert_eq!(col_1.next(), Some(&4));
    /// assert_eq!(col_1.next(), None);
    /// ```
    pub fn iter_nth_col(&self, n: usize) -> Result<VectorIter<&T>> {
        match self.order {
            Order::RowMajor => self.iter_nth_minor_axis_vector(n),
            Order::ColMajor => self.iter_nth_major_axis_vector(n),
        }
    }
}

impl<T> Matrix<T> {
    pub(super) unsafe fn iter_nth_row_unchecked(&self, n: usize) -> VectorIter<&T> {
        match self.order {
            Order::RowMajor => unsafe { self.iter_nth_major_axis_vector_unchecked(n) },
            Order::ColMajor => self.iter_nth_minor_axis_vector_unchecked(n),
        }
    }

    pub(super) unsafe fn iter_nth_col_unchecked(&self, n: usize) -> VectorIter<&T> {
        match self.order {
            Order::RowMajor => self.iter_nth_minor_axis_vector_unchecked(n),
            Order::ColMajor => unsafe { self.iter_nth_major_axis_vector_unchecked(n) },
        }
    }

    unsafe fn iter_nth_major_axis_vector_unchecked(&self, n: usize) -> VectorIter<&T> {
        let lower = n * self.major_stride();
        let upper = lower + self.major_stride();
        unsafe { Box::new(self.data.get_unchecked(lower..upper).iter()) }
    }

    fn iter_nth_major_axis_vector(&self, n: usize) -> Result<VectorIter<&T>> {
        if n >= self.major() {
            return Err(Error::IndexOutOfBounds);
        }
        unsafe { Ok(self.iter_nth_major_axis_vector_unchecked(n)) }
    }

    fn iter_nth_minor_axis_vector_unchecked(&self, n: usize) -> VectorIter<&T> {
        Box::new(self.data.iter().skip(n).step_by(self.major_stride()))
    }

    fn iter_nth_minor_axis_vector(&self, n: usize) -> Result<VectorIter<&T>> {
        if n >= self.minor() {
            return Err(Error::IndexOutOfBounds);
        }
        Ok(self.iter_nth_minor_axis_vector_unchecked(n))
    }

    fn iter_by_major_axis(&self) -> MatrixIter<&T> {
        Box::new((0..self.major()).map(|n| unsafe { self.iter_nth_major_axis_vector_unchecked(n) }))
    }

    fn iter_by_minor_axis(&self) -> MatrixIter<&T> {
        Box::new((0..self.minor()).map(|n| self.iter_nth_minor_axis_vector_unchecked(n)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn test_iter_rows() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];

        {
            let mut rows = matrix.iter_rows();
            let mut row_0 = rows.next().unwrap();
            assert_eq!(row_0.next(), Some(&0));
            assert_eq!(row_0.next(), Some(&1));
            assert_eq!(row_0.next(), Some(&2));
            assert_eq!(row_0.next(), None);
            let mut row_1 = rows.next().unwrap();
            assert_eq!(row_1.next(), Some(&3));
            assert_eq!(row_1.next(), Some(&4));
            assert_eq!(row_1.next(), Some(&5));
            assert_eq!(row_1.next(), None);
            assert!(rows.next().is_none());

            let mut rows = matrix.iter_rows();
            let mut row_1 = rows.next_back().unwrap();
            assert_eq!(row_1.next(), Some(&3));
            assert_eq!(row_1.next(), Some(&4));
            assert_eq!(row_1.next(), Some(&5));
            assert_eq!(row_1.next(), None);
            let mut row_0 = rows.next().unwrap();
            assert_eq!(row_0.next(), Some(&0));
            assert_eq!(row_0.next(), Some(&1));
            assert_eq!(row_0.next(), Some(&2));
            assert_eq!(row_0.next(), None);
            assert!(rows.next_back().is_none());
            assert!(rows.next().is_none());
        }

        matrix.switch_order();

        {
            let mut rows = matrix.iter_rows();
            let mut row_0 = rows.next().unwrap();
            assert_eq!(row_0.next(), Some(&0));
            assert_eq!(row_0.next(), Some(&1));
            assert_eq!(row_0.next(), Some(&2));
            assert_eq!(row_0.next(), None);
            let mut row_1 = rows.next().unwrap();
            assert_eq!(row_1.next(), Some(&3));
            assert_eq!(row_1.next(), Some(&4));
            assert_eq!(row_1.next(), Some(&5));
            assert_eq!(row_1.next(), None);
            assert!(rows.next().is_none());

            let mut rows = matrix.iter_rows();
            let mut row_1 = rows.next_back().unwrap();
            assert_eq!(row_1.next(), Some(&3));
            assert_eq!(row_1.next(), Some(&4));
            assert_eq!(row_1.next(), Some(&5));
            assert_eq!(row_1.next(), None);
            let mut row_0 = rows.next().unwrap();
            assert_eq!(row_0.next(), Some(&0));
            assert_eq!(row_0.next(), Some(&1));
            assert_eq!(row_0.next(), Some(&2));
            assert_eq!(row_0.next(), None);
            assert!(rows.next_back().is_none());
            assert!(rows.next().is_none());
        }
    }

    #[test]
    fn test_iter_cols() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];

        {
            let mut cols = matrix.iter_cols();
            let mut col_0 = cols.next().unwrap();
            assert_eq!(col_0.next(), Some(&0));
            assert_eq!(col_0.next(), Some(&3));
            assert_eq!(col_0.next(), None);
            let mut col_1 = cols.next().unwrap();
            assert_eq!(col_1.next(), Some(&1));
            assert_eq!(col_1.next(), Some(&4));
            assert_eq!(col_1.next(), None);
            let mut col_2 = cols.next().unwrap();
            assert_eq!(col_2.next(), Some(&2));
            assert_eq!(col_2.next(), Some(&5));
            assert_eq!(col_2.next(), None);
            assert!(cols.next().is_none());

            let mut cols = matrix.iter_cols();
            let mut col_2 = cols.next_back().unwrap();
            assert_eq!(col_2.next(), Some(&2));
            assert_eq!(col_2.next(), Some(&5));
            assert_eq!(col_2.next(), None);
            let mut col_0 = cols.next().unwrap();
            assert_eq!(col_0.next(), Some(&0));
            assert_eq!(col_0.next(), Some(&3));
            assert_eq!(col_0.next(), None);
            let mut col_1 = cols.next_back().unwrap();
            assert_eq!(col_1.next(), Some(&1));
            assert_eq!(col_1.next(), Some(&4));
            assert_eq!(col_1.next(), None);
            assert!(cols.next_back().is_none());
            assert!(cols.next().is_none());
        }

        matrix.switch_order();

        {
            let mut cols = matrix.iter_cols();
            let mut col_0 = cols.next().unwrap();
            assert_eq!(col_0.next(), Some(&0));
            assert_eq!(col_0.next(), Some(&3));
            assert_eq!(col_0.next(), None);
            let mut col_1 = cols.next().unwrap();
            assert_eq!(col_1.next(), Some(&1));
            assert_eq!(col_1.next(), Some(&4));
            assert_eq!(col_1.next(), None);
            let mut col_2 = cols.next().unwrap();
            assert_eq!(col_2.next(), Some(&2));
            assert_eq!(col_2.next(), Some(&5));
            assert_eq!(col_2.next(), None);
            assert!(cols.next().is_none());

            let mut cols = matrix.iter_cols();
            let mut col_2 = cols.next_back().unwrap();
            assert_eq!(col_2.next(), Some(&2));
            assert_eq!(col_2.next(), Some(&5));
            assert_eq!(col_2.next(), None);
            let mut col_0 = cols.next().unwrap();
            assert_eq!(col_0.next(), Some(&0));
            assert_eq!(col_0.next(), Some(&3));
            assert_eq!(col_0.next(), None);
            let mut col_1 = cols.next_back().unwrap();
            assert_eq!(col_1.next(), Some(&1));
            assert_eq!(col_1.next(), Some(&4));
            assert_eq!(col_1.next(), None);
            assert!(cols.next_back().is_none());
            assert!(cols.next().is_none());
        }
    }

    #[test]
    fn test_iter_nth_row() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];

        {
            let mut row_0 = matrix.iter_nth_row(0).unwrap();
            assert_eq!(row_0.next(), Some(&0));
            assert_eq!(row_0.next(), Some(&1));
            assert_eq!(row_0.next(), Some(&2));
            assert_eq!(row_0.next(), None);
            let mut row_1 = matrix.iter_nth_row(1).unwrap();
            assert_eq!(row_1.next(), Some(&3));
            assert_eq!(row_1.next(), Some(&4));
            assert_eq!(row_1.next(), Some(&5));
            assert_eq!(row_1.next(), None);
            assert!(matches!(
                matrix.iter_nth_row(2),
                Err(Error::IndexOutOfBounds)
            ));
        }

        matrix.switch_order();

        {
            let mut row_0 = matrix.iter_nth_row(0).unwrap();
            assert_eq!(row_0.next(), Some(&0));
            assert_eq!(row_0.next(), Some(&1));
            assert_eq!(row_0.next(), Some(&2));
            assert_eq!(row_0.next(), None);
            let mut row_1 = matrix.iter_nth_row(1).unwrap();
            assert_eq!(row_1.next(), Some(&3));
            assert_eq!(row_1.next(), Some(&4));
            assert_eq!(row_1.next(), Some(&5));
            assert_eq!(row_1.next(), None);
            assert!(matches!(
                matrix.iter_nth_row(2),
                Err(Error::IndexOutOfBounds)
            ));
        }
    }

    #[test]
    fn test_iter_nth_col() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];

        {
            let mut col_0 = matrix.iter_nth_col(0).unwrap();
            assert_eq!(col_0.next(), Some(&0));
            assert_eq!(col_0.next(), Some(&3));
            assert_eq!(col_0.next(), None);
            let mut col_1 = matrix.iter_nth_col(1).unwrap();
            assert_eq!(col_1.next(), Some(&1));
            assert_eq!(col_1.next(), Some(&4));
            assert_eq!(col_1.next(), None);
            let mut col_2 = matrix.iter_nth_col(2).unwrap();
            assert_eq!(col_2.next(), Some(&2));
            assert_eq!(col_2.next(), Some(&5));
            assert_eq!(col_2.next(), None);
            assert!(matches!(
                matrix.iter_nth_col(3),
                Err(Error::IndexOutOfBounds)
            ));
        }

        matrix.switch_order();

        {
            let mut col_0 = matrix.iter_nth_col(0).unwrap();
            assert_eq!(col_0.next(), Some(&0));
            assert_eq!(col_0.next(), Some(&3));
            assert_eq!(col_0.next(), None);
            let mut col_1 = matrix.iter_nth_col(1).unwrap();
            assert_eq!(col_1.next(), Some(&1));
            assert_eq!(col_1.next(), Some(&4));
            assert_eq!(col_1.next(), None);
            let mut col_2 = matrix.iter_nth_col(2).unwrap();
            assert_eq!(col_2.next(), Some(&2));
            assert_eq!(col_2.next(), Some(&5));
            assert_eq!(col_2.next(), None);
            assert!(matches!(
                matrix.iter_nth_col(3),
                Err(Error::IndexOutOfBounds)
            ));
        }
    }
}

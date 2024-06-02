use super::order::Order;
use super::Matrix;
use crate::error::{Error, Result};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// A trait object that represents a double-ended iterator over a vector.
pub type VectorIter<'a, T> = Box<dyn DoubleEndedIterator<Item = T> + 'a>;

/// A trait object that represents a double-ended iterator over a matrix.
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
            Order::RowMajor => Box::new(self.iter_by_major_axis()),
            Order::ColMajor => Box::new(self.iter_by_minor_axis()),
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
            Order::RowMajor => Box::new(self.iter_by_minor_axis()),
            Order::ColMajor => Box::new(self.iter_by_major_axis()),
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
            Order::RowMajor => Ok(Box::new(self.iter_nth_major_axis_vector(n)?)),
            Order::ColMajor => Ok(Box::new(self.iter_nth_minor_axis_vector(n)?)),
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
            Order::RowMajor => Ok(Box::new(self.iter_nth_minor_axis_vector(n)?)),
            Order::ColMajor => Ok(Box::new(self.iter_nth_major_axis_vector(n)?)),
        }
    }

    /// Returns an iterator over the elements of the matrix.
    ///
    /// # Notes
    ///
    /// The iteration order of elements is not guaranteed. In the current
    /// implementation, elements are iterated in memory order.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// let mut data: Vec<&i32> = matrix.iter_elements().collect();
    /// data.sort();  // order of elements is not guaranteed
    /// assert_eq!(data, vec![&0, &1, &2, &3, &4, &5]);
    /// ```
    pub fn iter_elements(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Returns an iterator that allows modifying each element
    /// of the matrix.
    ///
    /// # Notes
    ///
    /// The iteration order of elements is not guaranteed. In the current
    /// implementation, elements are iterated in memory order.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    ///
    /// let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// for element in matrix.iter_elements_mut() {
    ///     *element += 1;
    /// }
    /// assert_eq!(matrix, matrix![[1, 2, 3], [4, 5, 6]]);
    /// ```
    pub fn iter_elements_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    /// Creates a consuming iterator, that is, one that moves each
    /// element out of the matrix.
    ///
    /// # Notes
    ///
    /// The iteration order of elements is not guaranteed. In the current
    /// implementation, elements are iterated in memory order.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{matrix, Index};
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// let mut data: Vec<i32> = matrix.into_iter_elements().collect();
    /// data.sort();  // order of elements is not guaranteed
    /// assert_eq!(data, vec![0, 1, 2, 3, 4, 5]);
    /// ```
    pub fn into_iter_elements(self) -> impl Iterator<Item = T> {
        self.data.into_iter()
    }
}

#[cfg(feature = "rayon")]
impl<T> Matrix<T>
where
    T: Sync + Send,
{
    /// Returns a parallel iterator over the elements of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    /// use rayon::iter::ParallelIterator;
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// assert_eq!(matrix.par_iter_elements().sum::<i32>(), 15);
    /// ```
    pub fn par_iter_elements(&self) -> impl ParallelIterator<Item = &T> {
        self.data.par_iter()
    }

    /// Returns an parallel iterator that allows modifying each element
    /// of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::matrix;
    /// use rayon::iter::ParallelIterator;
    ///
    /// let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// matrix.par_iter_elements_mut().for_each(|element| *element += 1);
    /// assert_eq!(matrix, matrix![[1, 2, 3], [4, 5, 6]]);
    /// ```
    pub fn par_iter_elements_mut(&mut self) -> impl ParallelIterator<Item = &mut T> {
        self.data.par_iter_mut()
    }

    /// Creates a parallel consuming iterator, that is, one that moves each
    /// element out of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::{matrix, Index};
    /// use rayon::iter::ParallelIterator;
    ///
    /// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
    ///
    /// assert_eq!(matrix.into_par_iter_elements().sum::<i32>(), 15);
    /// ```
    pub fn into_par_iter_elements(self) -> impl ParallelIterator<Item = T> {
        self.data.into_par_iter()
    }
}

impl<T> Matrix<T> {
    pub(super) unsafe fn iter_nth_major_axis_vector_unchecked(
        &self,
        n: usize,
    ) -> impl DoubleEndedIterator<Item = &T> {
        let lower = n * self.major_stride();
        let upper = lower + self.major_stride();
        unsafe { self.data.get_unchecked(lower..upper).iter() }
    }

    pub(super) fn iter_nth_major_axis_vector(
        &self,
        n: usize,
    ) -> Result<impl DoubleEndedIterator<Item = &T>> {
        if n >= self.major() {
            return Err(Error::IndexOutOfBounds);
        }
        unsafe { Ok(self.iter_nth_major_axis_vector_unchecked(n)) }
    }

    pub(super) fn iter_by_major_axis(&self) -> impl DoubleEndedIterator<Item = VectorIter<&T>> {
        (0..self.major()).map(|n| -> VectorIter<&T> {
            unsafe { Box::new(self.iter_nth_major_axis_vector_unchecked(n)) }
        })
    }

    pub(super) fn iter_nth_minor_axis_vector_unchecked(
        &self,
        n: usize,
    ) -> impl DoubleEndedIterator<Item = &T> {
        self.data.iter().skip(n).step_by(self.major_stride())
    }

    pub(super) fn iter_nth_minor_axis_vector(
        &self,
        n: usize,
    ) -> Result<impl DoubleEndedIterator<Item = &T>> {
        if n >= self.minor() {
            return Err(Error::IndexOutOfBounds);
        }
        Ok(self.iter_nth_minor_axis_vector_unchecked(n))
    }

    pub(super) fn iter_by_minor_axis(&self) -> impl DoubleEndedIterator<Item = VectorIter<&T>> {
        (0..self.minor())
            .map(|n| -> VectorIter<&T> { Box::new(self.iter_nth_minor_axis_vector_unchecked(n)) })
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

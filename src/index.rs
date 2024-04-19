use crate::error::Error;
use crate::Matrix;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Index {
    pub row: usize,
    pub col: usize,
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Index({}, {})", self.row, self.col)
    }
}

impl From<(usize, usize)> for Index {
    fn from(value: (usize, usize)) -> Self {
        let (row, col) = value;
        Self { row, col }
    }
}

impl From<[usize; 2]> for Index {
    fn from(value: [usize; 2]) -> Self {
        let [row, col] = value;
        Self { row, col }
    }
}

impl<T> Matrix<T> {
    fn flatten_index<I: Into<Index>>(&self, index: I) -> usize {
        let index: Index = index.into();
        if index.row >= self.nrows() || index.col >= self.ncols() {
            panic!("{}", Error::IndexOutOfBounds);
        }
        index.row * self.ncols() + index.col
    }
}

impl<T, I> std::ops::Index<I> for Matrix<T>
where
    I: Into<Index>,
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        let index = self.flatten_index(index);
        &self.data[index]
    }
}

impl<T, I> std::ops::IndexMut<I> for Matrix<T>
where
    I: Into<Index>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let index = self.flatten_index(index);
        &mut self.data[index]
    }
}

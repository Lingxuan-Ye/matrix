use crate::Matrix;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Index(pub usize, pub usize);

impl Index {
    pub fn row(&self) -> usize {
        self.0
    }

    pub fn col(&self) -> usize {
        self.1
    }
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row(), self.col())
    }
}

impl From<(usize, usize)> for Index {
    fn from(value: (usize, usize)) -> Self {
        let (row, col) = value;
        Self(row, col)
    }
}

impl From<[usize; 2]> for Index {
    fn from(value: [usize; 2]) -> Self {
        let [row, col] = value;
        Self(row, col)
    }
}

impl<T> Matrix<T> {
    fn flatten_index<I: Into<Index>>(&self, index: I) -> usize {
        let index: Index = index.into();
        if index.row() >= self.nrows() || index.col() >= self.ncols() {
            panic!("index out of bounds");
        }
        index.row() * self.ncols() + index.col()
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::matrix;

    #[test]
    fn test_display() {
        assert_eq!(Index(2, 3).to_string(), "(2, 3)");
        assert_eq!(Index(3, 2).to_string(), "(3, 2)");
    }

    #[test]
    fn test_from_trait() {
        let target = Index(2, 3);

        assert_eq!(Index::from(Index(2, 3)), target);
        assert_ne!(Index::from(Index(3, 2)), target);

        assert_eq!(Index::from((2, 3)), target);
        assert_ne!(Index::from((3, 2)), target);

        assert_eq!(Index::from([2, 3]), target);
        assert_ne!(Index::from([3, 2]), target);
    }

    #[test]
    fn test_index() {
        let matrix = matrix![[0, 1, 2], [3, 4, 5]];
        assert_eq!(matrix[(0, 0)], 0);
        assert_eq!(matrix[(0, 1)], 1);
        assert_eq!(matrix[(0, 2)], 2);
        assert_eq!(matrix[(1, 0)], 3);
        assert_eq!(matrix[(1, 1)], 4);
        assert_eq!(matrix[(1, 2)], 5);
    }

    #[test]
    fn test_index_mut() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
        matrix[(0, 0)] += 1;
        matrix[(0, 1)] += 1;
        matrix[(0, 2)] += 1;
        matrix[(1, 0)] += 1;
        matrix[(1, 1)] += 1;
        matrix[(1, 2)] += 1;
        assert_eq!(matrix, matrix![[1, 2, 3], [4, 5, 6]])
    }

    #[test]
    #[should_panic]
    fn test_row_out_of_bounds() {
        let matrix = matrix![[0, 1, 2], [3, 4, 5]];
        matrix[(2, 0)];
    }

    #[test]
    #[should_panic]
    fn test_col_out_of_bounds() {
        let matrix = matrix![[0, 1, 2], [3, 4, 5]];
        matrix[(0, 3)];
    }
}

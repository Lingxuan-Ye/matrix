use super::order::Order;
use super::shape::AxisShape;
use super::Matrix;
use crate::error::{Error, Result};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Index {
    pub row: usize,
    pub col: usize,
}

impl Index {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct AxisIndex {
    pub major: usize,
    pub minor: usize,
}

impl AxisIndex {
    pub fn new<I: Into<Index>>(index: I, order: Order) -> Self {
        let index: Index = index.into();
        let (major, minor) = match order {
            Order::RowMajor => (index.row, index.col),
            Order::ColMajor => (index.col, index.row),
        };
        Self { major, minor }
    }

    pub fn is_out_of_bounds(&self, shape: AxisShape) -> bool {
        self.major >= shape.major() || self.minor >= shape.minor()
    }

    pub fn transpose(&mut self) -> &mut Self {
        (self.major, self.minor) = (self.minor, self.major);
        self
    }

    pub fn from_flattened_unchecked(index: usize, shape: AxisShape) -> Self {
        let major = index / shape.major_stride();
        // let minor = (index % shape.major_stride()) / shape.minor();
        let minor = index % shape.major_stride();
        Self { major, minor }
    }

    pub fn try_from_flattened(index: usize, shape: AxisShape) -> Result<Self> {
        if index >= shape.size() {
            return Err(Error::IndexOutOfBounds);
        }
        Ok(Self::from_flattened_unchecked(index, shape))
    }

    pub fn from_flattened(index: usize, shape: AxisShape) -> Self {
        match Self::try_from_flattened(index, shape) {
            Ok(index) => index,
            Err(error) => panic!("{error}"),
        }
    }

    pub fn flatten_for_unchecked(&self, shape: AxisShape) -> usize {
        // self.major * shape.major_stride() + self.minor * shape.minor_stride()
        self.major * shape.major_stride() + self.minor
    }

    pub fn try_flatten_for(&self, shape: AxisShape) -> Result<usize> {
        if self.is_out_of_bounds(shape) {
            return Err(Error::IndexOutOfBounds);
        }
        Ok(Self::flatten_for_unchecked(self, shape))
    }

    pub fn flatten_for(&self, shape: AxisShape) -> usize {
        match self.try_flatten_for(shape) {
            Ok(index) => index,
            Err(error) => panic!("{error}"),
        }
    }

    pub fn interpret_with(&self, order: Order) -> Index {
        let (row, col) = match order {
            Order::RowMajor => (self.major, self.minor),
            Order::ColMajor => (self.minor, self.major),
        };
        Index { row, col }
    }

    pub fn interpret_row_with(&self, order: Order) -> usize {
        match order {
            Order::RowMajor => self.major,
            Order::ColMajor => self.minor,
        }
    }

    pub fn interpret_col_with(&self, order: Order) -> usize {
        match order {
            Order::RowMajor => self.minor,
            Order::ColMajor => self.major,
        }
    }
}

impl<T> Matrix<T> {
    pub fn get<I: Into<Index>>(&self, index: I) -> Option<&T> {
        let index = AxisIndex::new(index, self.order)
            .try_flatten_for(self.shape)
            .ok()?;
        self.data.get(index)
    }

    pub fn get_mut<I: Into<Index>>(&mut self, index: I) -> Option<&mut T> {
        let index = AxisIndex::new(index, self.order)
            .try_flatten_for(self.shape)
            .ok()?;
        self.data.get_mut(index)
    }
}

impl<T, I> std::ops::Index<I> for Matrix<T>
where
    I: Into<Index>,
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        let index = AxisIndex::new(index, self.order).flatten_for(self.shape);
        &self.data[index]
    }
}

impl<T, I> std::ops::IndexMut<I> for Matrix<T>
where
    I: Into<Index>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let index = AxisIndex::new(index, self.order).flatten_for(self.shape);
        &mut self.data[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::matrix;

    #[test]
    fn test_index_new() {
        let target = Index { row: 2, col: 3 };

        assert_eq!(Index::new(2, 3), target);
        assert_ne!(Index::new(3, 2), target);
    }

    #[test]
    fn test_index_display() {
        assert_eq!(Index::new(2, 3).to_string(), "(2, 3)");
        assert_eq!(Index::new(3, 2).to_string(), "(3, 2)");
    }

    #[test]
    fn test_index_from() {
        let target = Index { row: 2, col: 3 };

        assert_eq!(Index::from((2, 3)), target);
        assert_ne!(Index::from((3, 2)), target);

        assert_eq!(Index::from([2, 3]), target);
        assert_ne!(Index::from([3, 2]), target);
    }

    #[test]
    fn test_axis_index_new() {
        assert_eq!(
            AxisIndex::new((2, 3), Order::RowMajor),
            AxisIndex { major: 2, minor: 3 }
        );
        assert_eq!(
            AxisIndex::new((2, 3), Order::ColMajor),
            AxisIndex { major: 3, minor: 2 }
        );
        assert_eq!(
            AxisIndex::new((3, 2), Order::RowMajor),
            AxisIndex { major: 3, minor: 2 }
        );
    }

    #[test]
    fn test_axis_index_is_out_of_bounds() {
        assert!(!AxisIndex { major: 1, minor: 0 }.is_out_of_bounds(shape(2, 3)));
        assert!(AxisIndex { major: 2, minor: 3 }.is_out_of_bounds(shape(2, 3)));
    }

    #[test]
    fn test_axis_index_transpose() {
        let mut index = AxisIndex { major: 2, minor: 3 };

        index.transpose();
        assert_eq!(index, AxisIndex { major: 3, minor: 2 });

        index.transpose();
        assert_eq!(index, AxisIndex { major: 2, minor: 3 });
    }

    fn shape(major: usize, minor: usize) -> AxisShape {
        AxisShape::build((major, minor), Order::RowMajor).unwrap()
    }

    #[test]
    fn test_axis_index_from_flattened_unchecked() {
        assert_eq!(
            AxisIndex::from_flattened_unchecked(3, shape(2, 3)),
            AxisIndex { major: 1, minor: 0 }
        );
        assert_eq!(
            AxisIndex::from_flattened_unchecked(3, shape(3, 2)),
            AxisIndex { major: 1, minor: 1 }
        );
        // out of bounds
        assert_eq!(
            AxisIndex::from_flattened_unchecked(6, shape(2, 3)),
            AxisIndex { major: 2, minor: 0 }
        );
    }

    #[test]
    fn test_axis_index_try_from_flattened() {
        assert_eq!(
            AxisIndex::try_from_flattened(3, shape(2, 3)),
            Ok(AxisIndex { major: 1, minor: 0 })
        );
        assert_eq!(
            AxisIndex::try_from_flattened(3, shape(3, 2)),
            Ok(AxisIndex { major: 1, minor: 1 })
        );
        assert_eq!(
            AxisIndex::try_from_flattened(6, shape(2, 3)),
            Err(Error::IndexOutOfBounds)
        );
    }

    #[test]
    fn test_axis_index_from_flattened() {
        assert_eq!(
            AxisIndex::from_flattened(3, shape(2, 3)),
            AxisIndex { major: 1, minor: 0 }
        );
        assert_eq!(
            AxisIndex::from_flattened(3, shape(3, 2)),
            AxisIndex { major: 1, minor: 1 }
        );
    }

    #[test]
    #[should_panic]
    fn test_axis_index_from_flattened_fails() {
        AxisIndex::from_flattened(6, shape(2, 3));
    }

    #[test]
    fn test_axis_index_flatten_for_unchecked() {
        assert_eq!(
            AxisIndex { major: 1, minor: 1 }.flatten_for_unchecked(shape(2, 3)),
            4
        );
        assert_eq!(
            AxisIndex { major: 1, minor: 1 }.flatten_for_unchecked(shape(3, 2)),
            3
        );
        // out of bounds
        assert_eq!(
            AxisIndex { major: 2, minor: 3 }.flatten_for_unchecked(shape(2, 3)),
            9
        );
    }

    #[test]
    fn test_axis_index_try_flatten_for() {
        assert_eq!(
            AxisIndex { major: 1, minor: 1 }.try_flatten_for(shape(2, 3)),
            Ok(4)
        );
        assert_eq!(
            AxisIndex { major: 1, minor: 1 }.try_flatten_for(shape(3, 2)),
            Ok(3)
        );
        assert_eq!(
            AxisIndex { major: 2, minor: 3 }.try_flatten_for(shape(2, 3)),
            Err(Error::IndexOutOfBounds)
        );
    }

    #[test]
    fn test_axis_index_flatten_for() {
        assert_eq!(AxisIndex { major: 1, minor: 1 }.flatten_for(shape(2, 3)), 4);
        assert_eq!(AxisIndex { major: 1, minor: 1 }.flatten_for(shape(3, 2)), 3);
    }

    #[test]
    #[should_panic]
    fn test_axis_index_flatten_for_fails() {
        AxisIndex { major: 2, minor: 3 }.flatten_for(shape(2, 3));
    }

    #[test]
    fn test_axis_index_interpret() {
        let index = AxisIndex { major: 2, minor: 3 };

        assert_eq!(index.interpret_with(Order::RowMajor), Index::new(2, 3));
        assert_eq!(index.interpret_with(Order::ColMajor), Index::new(3, 2));

        assert_eq!(index.interpret_row_with(Order::RowMajor), 2);
        assert_eq!(index.interpret_row_with(Order::ColMajor), 3);

        assert_eq!(index.interpret_col_with(Order::RowMajor), 3);
        assert_eq!(index.interpret_col_with(Order::ColMajor), 2);
    }

    #[test]
    fn test_matrix_get() {
        let matrix = matrix![[0, 1, 2], [3, 4, 5]];
        assert_eq!(matrix.get((0, 0)), Some(&0));
        assert_eq!(matrix.get((0, 1)), Some(&1));
        assert_eq!(matrix.get((0, 2)), Some(&2));
        assert_eq!(matrix.get((1, 0)), Some(&3));
        assert_eq!(matrix.get((1, 1)), Some(&4));
        assert_eq!(matrix.get((1, 2)), Some(&5));
        assert_eq!(matrix.get((2, 0)), None);
    }

    #[test]
    fn test_matrix_get_mut() {
        let mut matrix = matrix![[0, 1, 2], [3, 4, 5]];
        assert_eq!(matrix.get_mut((0, 0)), Some(&mut 0));
        assert_eq!(matrix.get_mut((0, 1)), Some(&mut 1));
        assert_eq!(matrix.get_mut((0, 2)), Some(&mut 2));
        assert_eq!(matrix.get_mut((1, 0)), Some(&mut 3));
        assert_eq!(matrix.get_mut((1, 1)), Some(&mut 4));
        assert_eq!(matrix.get_mut((1, 2)), Some(&mut 5));
        assert_eq!(matrix.get_mut((2, 0)), None);
    }

    #[test]
    fn test_matrix_index() {
        let matrix = matrix![[0, 1, 2], [3, 4, 5]];
        assert_eq!(matrix[(0, 0)], 0);
        assert_eq!(matrix[(0, 1)], 1);
        assert_eq!(matrix[(0, 2)], 2);
        assert_eq!(matrix[(1, 0)], 3);
        assert_eq!(matrix[(1, 1)], 4);
        assert_eq!(matrix[(1, 2)], 5);
    }

    #[test]
    fn test_matrix_index_mut() {
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

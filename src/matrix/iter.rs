use super::order::Order;
use super::Matrix;

pub type VectorIter<'a, T> = Box<dyn Iterator<Item = T> + 'a>;
pub type MatrixIter<'a, T> = Box<dyn Iterator<Item = VectorIter<'a, T>> + 'a>;

#[derive(Clone, Debug)]
struct Count {
    total: usize,
    forth: usize,
    back: usize,
}

impl Count {
    fn new(total: usize) -> Self {
        Self {
            total,
            forth: 0,
            back: 0,
        }
    }

    fn is_reached(&self) -> bool {
        self.forth + self.back >= self.total
    }
}

#[derive(Clone, Debug)]
struct MajorAxisIter<'a, T> {
    matrix: &'a Matrix<T>,
    count: Count,
}

impl<'a, T> MajorAxisIter<'a, T> {
    fn new(matrix: &'a Matrix<T>) -> Self {
        let total = matrix.major();
        let count = Count::new(total);
        Self { matrix, count }
    }
}

impl<'a, T> Iterator for MajorAxisIter<'a, T> {
    type Item = VectorIter<'a, &'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count.is_reached() {
            return None;
        }

        let major_stride = self.matrix.major_stride();
        let start = self.count.forth * major_stride;
        let end = start + major_stride;
        let iter = self.matrix.data[start..end].iter();

        self.count.forth += 1;

        Some(Box::new(iter))
    }
}

impl<'a, T> DoubleEndedIterator for MajorAxisIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.count.is_reached() {
            return None;
        }

        let major_stride = self.matrix.major_stride();
        let end = (self.count.total - self.count.back) * major_stride;
        let start = end - major_stride;
        let iter = self.matrix.data[start..end].iter();

        self.count.back += 1;

        Some(Box::new(iter))
    }
}

#[derive(Clone, Debug)]
struct MinorAxisIter<'a, T> {
    matrix: &'a Matrix<T>,
    count: Count,
}

impl<'a, T> MinorAxisIter<'a, T> {
    fn new(matrix: &'a Matrix<T>) -> Self {
        let total = matrix.minor();
        let count = Count::new(total);
        Self { matrix, count }
    }
}

impl<'a, T> Iterator for MinorAxisIter<'a, T> {
    type Item = VectorIter<'a, &'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count.is_reached() {
            return None;
        }

        let major_stride = self.matrix.major_stride();
        let iter = self
            .matrix
            .data
            .iter()
            .skip(self.count.forth)
            .step_by(major_stride);

        self.count.forth += 1;

        Some(Box::new(iter))
    }
}

impl<'a, T> DoubleEndedIterator for MinorAxisIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.count.is_reached() {
            return None;
        }

        let major_stride = self.matrix.major_stride();
        let iter = self
            .matrix
            .data
            .iter()
            .skip(self.count.total - self.count.back - 1)
            .step_by(major_stride);

        self.count.back += 1;

        Some(Box::new(iter))
    }
}

impl<T> Matrix<T> {
    pub fn iter_rows(&self) -> MatrixIter<&T> {
        match self.order {
            Order::RowMajor => Box::new(MajorAxisIter::new(self)),
            Order::ColMajor => Box::new(MinorAxisIter::new(self)),
        }
    }

    pub fn iter_cols(&self) -> MatrixIter<&T> {
        match self.order {
            Order::RowMajor => Box::new(MinorAxisIter::new(self)),
            Order::ColMajor => Box::new(MajorAxisIter::new(self)),
        }
    }
}

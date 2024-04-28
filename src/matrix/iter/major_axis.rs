use super::super::Matrix;
use super::count::{BackwardGuard, Counter, ForwardGuard};
use super::VectorIter;
use std::slice::Iter;

#[derive(Clone, Debug)]
pub(super) struct MajorAxisVectorIter<'a, T>(Iter<'a, T>);

impl<'a, T> MajorAxisVectorIter<'a, T> {
    pub fn new(matrix: &'a Matrix<T>, n: usize) -> Self {
        let major_stride = matrix.major_stride();
        let start = n * major_stride;
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

#[derive(Clone, Debug)]
pub(super) struct MajorAxisMatrixIter<'a, T> {
    matrix: &'a Matrix<T>,
    counter: Counter,
}

impl<'a, T> MajorAxisMatrixIter<'a, T> {
    pub fn new(matrix: &'a Matrix<T>) -> Self {
        let total = matrix.major();
        let counter = Counter::new(total);
        Self { matrix, counter }
    }
}

impl<'a, T> Iterator for MajorAxisMatrixIter<'a, T> {
    type Item = VectorIter<'a, &'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        let guard = ForwardGuard::build(&mut self.counter)?;
        let iter = MajorAxisVectorIter::new(self.matrix, guard.pointer());
        Some(Box::new(iter))
    }
}

impl<'a, T> DoubleEndedIterator for MajorAxisMatrixIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let guard = BackwardGuard::build(&mut self.counter)?;
        let iter = MajorAxisVectorIter::new(self.matrix, guard.pointer());
        Some(Box::new(iter))
    }
}

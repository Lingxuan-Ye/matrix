use super::super::Matrix;
use super::count::{BackwardGuard, Counter, ForwardGuard};
use super::VectorIter;
use std::iter::{Skip, StepBy};
use std::slice::Iter;

#[derive(Clone, Debug)]
pub(super) struct MinorAxisVectorIter<'a, T>(StepBy<Skip<Iter<'a, T>>>);

impl<'a, T> MinorAxisVectorIter<'a, T> {
    pub fn new(matrix: &'a Matrix<T>, n: usize) -> Self {
        let major_stride = matrix.major_stride();
        let iter = matrix.data.iter().skip(n).step_by(major_stride);
        Self(iter)
    }
}

impl<'a, T> Iterator for MinorAxisVectorIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[derive(Clone, Debug)]
pub(super) struct MinorAxisMatrixIter<'a, T> {
    matrix: &'a Matrix<T>,
    counter: Counter,
}

impl<'a, T> MinorAxisMatrixIter<'a, T> {
    pub fn new(matrix: &'a Matrix<T>) -> Self {
        let total = matrix.minor();
        let counter = Counter::new(total);
        Self { matrix, counter }
    }
}

impl<'a, T> Iterator for MinorAxisMatrixIter<'a, T> {
    type Item = VectorIter<'a, &'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        let guard = ForwardGuard::build(&mut self.counter)?;
        let iter = MinorAxisVectorIter::new(self.matrix, guard.pointer());
        Some(Box::new(iter))
    }
}

impl<'a, T> DoubleEndedIterator for MinorAxisMatrixIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let guard = BackwardGuard::build(&mut self.counter)?;
        let iter = MinorAxisVectorIter::new(self.matrix, guard.pointer());
        Some(Box::new(iter))
    }
}

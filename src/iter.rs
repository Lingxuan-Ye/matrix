use std::iter::StepBy;
use std::ops::Range;

pub type VectorIter<'a, T> = Box<dyn DoubleEndedIterator<Item = T> + 'a>;
pub type MatrixIter<'a, T> = Box<dyn DoubleEndedIterator<Item = VectorIter<'a, T>> + 'a>;

pub type StepRange<Idx> = StepBy<Range<Idx>>;

use crate::error::{Error, Result};
use std::iter::StepBy;
use std::num::NonZeroUsize;
use std::ops::Range;

pub type VectorIter<'a, T> = Box<dyn DoubleEndedIterator<Item = T> + 'a>;
pub type MatrixIter<'a, T> = Box<dyn DoubleEndedIterator<Item = VectorIter<'a, T>> + 'a>;

#[derive(Clone, Debug)]
pub struct StepRange {
    pub start: usize,
    pub end: usize,
    pub step: NonZeroUsize,
}

impl StepRange {
    pub fn new(end: usize) -> Self {
        Self {
            start: 0,
            end,
            step: unsafe { NonZeroUsize::new_unchecked(1) },
        }
    }

    pub fn start(mut self, start: usize) -> Self {
        self.start = start;
        self
    }

    pub fn end(mut self, end: usize) -> Self {
        self.end = end;
        self
    }

    pub fn step(mut self, step: usize) -> Result<Self> {
        self.step = NonZeroUsize::new(step).ok_or(Error::ZeroStep)?;
        Ok(self)
    }
}

impl IntoIterator for StepRange {
    type Item = usize;
    type IntoIter = StepBy<Range<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        (self.start..self.end).step_by(self.step.get())
    }
}

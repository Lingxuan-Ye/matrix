use crate::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct StepRange {
    lower: usize,
    upper: usize,
    step: usize,
}

impl StepRange {
    pub fn build(lower: usize, upper: usize, step: usize) -> Result<Self> {
        if step == 0 {
            return Err(Error::ZeroStep);
        }
        let upper = (upper - 1) - ((upper - 1) - lower) % step + 1;
        Ok(Self { lower, upper, step })
    }

    fn is_reached(&self) -> bool {
        self.lower >= self.upper
    }
}

impl Iterator for StepRange {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_reached() {
            return None;
        }
        let pointer = self.lower;
        self.lower = self.lower.saturating_add(self.step);
        Some(pointer)
    }
}

impl DoubleEndedIterator for StepRange {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.is_reached() {
            return None;
        }
        let pointer = self.upper - 1;
        self.upper = self.upper.saturating_sub(self.step);
        Some(pointer)
    }
}

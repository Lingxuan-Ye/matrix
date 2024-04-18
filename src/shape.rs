use crate::error::{Error, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shape {
    nrows: usize,
    ncols: usize,
}

impl Shape {
    pub fn build(nrows: usize, ncols: usize) -> Result<Self> {
        if nrows == 0 || ncols == 0 {
            return Err(Error::ZeroSize);
        }
        nrows.checked_mul(ncols).ok_or(Error::SizeOverflow)?;
        Ok(Self { nrows, ncols })
    }

    pub fn nrows(&self) -> usize {
        self.nrows
    }

    pub fn ncols(&self) -> usize {
        self.ncols
    }

    pub fn size(&self) -> usize {
        self.nrows * self.ncols
    }
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Shape({}, {})", self.nrows, self.ncols)
    }
}

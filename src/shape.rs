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

pub trait TryIntoShape {
    fn try_into_shape(self) -> Result<Shape>;
}

impl TryIntoShape for Shape {
    fn try_into_shape(self) -> Result<Shape> {
        Ok(self)
    }
}

impl TryIntoShape for (usize, usize) {
    fn try_into_shape(self) -> Result<Shape> {
        let (nrows, ncols) = self;
        Shape::build(nrows, ncols)
    }
}

impl TryIntoShape for [usize; 2] {
    fn try_into_shape(self) -> Result<Shape> {
        let [nrows, ncols] = self;
        Shape::build(nrows, ncols)
    }
}

impl TryFrom<(usize, usize)> for Shape {
    type Error = Error;

    fn try_from(value: (usize, usize)) -> Result<Self> {
        value.try_into_shape()
    }
}

impl TryFrom<[usize; 2]> for Shape {
    type Error = Error;

    fn try_from(value: [usize; 2]) -> Result<Self> {
        value.try_into_shape()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build() {
        assert_eq!(Shape::build(0, 0).unwrap_err(), Error::ZeroSize);
        assert_eq!(Shape::build(0, 1).unwrap_err(), Error::ZeroSize);
        assert_eq!(Shape::build(1, 0).unwrap_err(), Error::ZeroSize);

        assert_eq!(
            Shape::build(usize::MAX, 2).unwrap_err(),
            Error::SizeOverflow
        );
        assert_eq!(
            Shape::build(2, usize::MAX).unwrap_err(),
            Error::SizeOverflow
        );

        let shape = Shape { nrows: 2, ncols: 3 };
        assert_eq!(Shape::build(2, 3).unwrap(), shape);
        assert_ne!(Shape::build(3, 2).unwrap(), shape);
    }

    #[test]
    fn test_display() {
        assert_eq!(Shape::build(2, 3).unwrap().to_string(), "Shape(2, 3)");
        assert_eq!(Shape::build(3, 2).unwrap().to_string(), "Shape(3, 2)");
    }

    #[test]
    fn test_try_into_shape() {
        let shape = Shape { nrows: 2, ncols: 3 };

        assert_eq!((0, 1).try_into_shape().unwrap_err(), Error::ZeroSize);
        assert_eq!((2, 3).try_into_shape().unwrap(), shape);
        assert_ne!((3, 2).try_into_shape().unwrap(), shape);

        assert_eq!([0, 1].try_into_shape().unwrap_err(), Error::ZeroSize);
        assert_eq!([2, 3].try_into_shape().unwrap(), shape);
        assert_ne!([3, 2].try_into_shape().unwrap(), shape);

        assert_eq!(
            Shape { nrows: 2, ncols: 3 }.try_into_shape().unwrap(),
            shape
        );
        assert_ne!(
            Shape { nrows: 3, ncols: 2 }.try_into_shape().unwrap(),
            shape
        );
    }
}

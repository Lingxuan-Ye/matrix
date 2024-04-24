use super::order::Order;
use crate::error::{Error, Result};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Shape {
    nrows: usize,
    ncols: usize,
}

impl Shape {
    pub fn build(nrows: usize, ncols: usize) -> Result<Self> {
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
        write!(f, "({}, {})", self.nrows, self.ncols)
    }
}

impl TryFrom<(usize, usize)> for Shape {
    type Error = Error;

    fn try_from(value: (usize, usize)) -> Result<Self> {
        let (nrows, ncols) = value;
        Self::build(nrows, ncols)
    }
}

impl TryFrom<[usize; 2]> for Shape {
    type Error = Error;

    fn try_from(value: [usize; 2]) -> Result<Self> {
        let [nrows, ncols] = value;
        Self::build(nrows, ncols)
    }
}

pub trait TryIntoShape {
    fn try_into_shape(self) -> Result<Shape>;
}

impl<S> TryIntoShape for S
where
    S: TryInto<Shape, Error = Error>,
{
    fn try_into_shape(self) -> Result<Shape> {
        self.try_into()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct AxisShape {
    major: usize,
    minor: usize,
}

impl AxisShape {
    pub fn build<S: TryIntoShape>(shape: S, order: Order) -> Result<Self> {
        let shape = shape.try_into_shape()?;
        let (major, minor) = match order {
            Order::RowMajor => (shape.nrows, shape.ncols),
            Order::ColMajor => (shape.ncols, shape.nrows),
        };
        Ok(Self { major, minor })
    }

    pub fn interpret_with(&self, order: Order) -> Shape {
        let (nrows, ncols) = match order {
            Order::RowMajor => (self.major, self.minor),
            Order::ColMajor => (self.minor, self.major),
        };
        Shape { nrows, ncols }
    }

    pub fn interpret_nrows_with(&self, order: Order) -> usize {
        match order {
            Order::RowMajor => self.major,
            Order::ColMajor => self.minor,
        }
    }

    pub fn interpret_ncols_with(&self, order: Order) -> usize {
        match order {
            Order::RowMajor => self.minor,
            Order::ColMajor => self.major,
        }
    }

    pub fn major(&self) -> usize {
        self.major
    }

    pub fn minor(&self) -> usize {
        self.minor
    }

    pub fn size(&self) -> usize {
        self.major * self.minor
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::shape;

    #[test]
    fn test_build_shape() {
        let target = Shape { nrows: 2, ncols: 3 };

        assert_eq!(Shape::build(2, 3), Ok(target));
        assert_ne!(Shape::build(3, 2), Ok(target));
        assert_eq!(Shape::build(usize::MAX, 2), Err(Error::SizeOverflow));
    }

    #[test]
    fn test_display_shape() {
        assert_eq!(shape!(2, 3).to_string(), "(2, 3)");
        assert_eq!(shape!(3, 2).to_string(), "(3, 2)");
    }

    #[test]
    fn test_try_into_shape() {
        let target = Shape { nrows: 2, ncols: 3 };

        assert_eq!((2, 3).try_into_shape(), Ok(target));
        assert_ne!((3, 2).try_into_shape(), Ok(target));
        assert_eq!((usize::MAX, 2).try_into_shape(), Err(Error::SizeOverflow));

        assert_eq!([2, 3].try_into_shape(), Ok(target));
        assert_ne!([3, 2].try_into_shape(), Ok(target));
        assert_eq!([usize::MAX, 2].try_into_shape(), Err(Error::SizeOverflow));
    }

    #[test]
    fn test_build_axis_shape() {
        let target = AxisShape { major: 2, minor: 3 };

        assert_eq!(AxisShape::build((2, 3), Order::RowMajor), Ok(target));
        assert_ne!(AxisShape::build((2, 3), Order::ColMajor), Ok(target));
        assert_eq!(AxisShape::build((3, 2), Order::ColMajor), Ok(target));

        assert_eq!(
            AxisShape::build((usize::MAX, 2), Order::RowMajor),
            Err(Error::SizeOverflow)
        );
        assert_eq!(
            AxisShape::build((usize::MAX, 2), Order::ColMajor),
            Err(Error::SizeOverflow)
        );
    }
}

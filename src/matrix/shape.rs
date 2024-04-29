use super::order::Order;
use crate::error::{Error, Result};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Shape {
    pub nrows: usize,
    pub ncols: usize,
}

impl Shape {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        Self { nrows, ncols }
    }
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.nrows, self.ncols)
    }
}

pub trait ShapeLike {
    fn nrows(&self) -> usize;

    fn ncols(&self) -> usize;

    fn size(&self) -> Option<usize> {
        self.nrows().checked_mul(self.ncols())
    }
}

impl ShapeLike for Shape {
    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

impl ShapeLike for (usize, usize) {
    fn nrows(&self) -> usize {
        self.0
    }

    fn ncols(&self) -> usize {
        self.1
    }
}

impl ShapeLike for [usize; 2] {
    fn nrows(&self) -> usize {
        self[0]
    }

    fn ncols(&self) -> usize {
        self[1]
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct AxisShape {
    major: usize,
    minor: usize,
}

impl AxisShape {
    pub fn build<S: ShapeLike>(shape: S, order: Order) -> Result<Self> {
        if shape.size().is_none() {
            return Err(Error::SizeOverflow);
        }
        let (major, minor) = match order {
            Order::RowMajor => (shape.nrows(), shape.ncols()),
            Order::ColMajor => (shape.ncols(), shape.nrows()),
        };
        Ok(Self { major, minor })
    }

    pub fn major(&self) -> usize {
        self.major
    }

    pub fn minor(&self) -> usize {
        self.minor
    }

    pub fn major_stride(&self) -> usize {
        self.minor
    }

    pub const fn minor_stride(&self) -> usize {
        1
    }

    pub fn size(&self) -> usize {
        self.major * self.minor
    }

    pub fn transpose(&mut self) -> &mut Self {
        (self.major, self.minor) = (self.minor, self.major);
        self
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shape_new() {
        let target = Shape { nrows: 2, ncols: 3 };

        assert_eq!(Shape::new(2, 3), target);
        assert_ne!(Shape::new(3, 2), target);
    }

    #[test]
    fn test_shape_display() {
        assert_eq!(Shape::new(2, 3).to_string(), "(2, 3)");
        assert_eq!(Shape::new(3, 2).to_string(), "(3, 2)");
    }

    #[test]
    fn test_shape_like() {
        assert_eq!(Shape::new(2, 3).nrows(), 2);
        assert_eq!(Shape::new(2, 3).ncols(), 3);
        assert_eq!(Shape::new(2, 3).size(), Some(6));
        assert_eq!(Shape::new(2, usize::MAX).size(), None);

        assert_eq!((2, 3).nrows(), 2);
        assert_eq!((2, 3).ncols(), 3);
        assert_eq!((2, 3).size(), Some(6));
        assert_eq!((2, usize::MAX).size(), None);

        assert_eq!([2, 3].nrows(), 2);
        assert_eq!([2, 3].ncols(), 3);
        assert_eq!([2, 3].size(), Some(6));
        assert_eq!([2, usize::MAX].size(), None);
    }

    #[test]
    fn test_axis_shape_build() {
        assert_eq!(
            AxisShape::build((2, 3), Order::RowMajor),
            Ok(AxisShape { major: 2, minor: 3 })
        );
        assert_eq!(
            AxisShape::build((2, 3), Order::ColMajor),
            Ok(AxisShape { major: 3, minor: 2 })
        );

        assert_eq!(
            AxisShape::build((3, 2), Order::RowMajor),
            Ok(AxisShape { major: 3, minor: 2 })
        );
        assert_eq!(
            AxisShape::build((2, usize::MAX), Order::RowMajor),
            Err(Error::SizeOverflow)
        );
    }

    #[test]
    fn test_axis_shape_major_stride() {
        assert_eq!(AxisShape { major: 2, minor: 3 }.major_stride(), 3);
        assert_eq!(AxisShape { major: 3, minor: 2 }.major_stride(), 2);
    }

    #[test]
    fn test_axis_shape_minor_stride() {
        assert_eq!(AxisShape { major: 2, minor: 3 }.minor_stride(), 1);
        assert_eq!(AxisShape { major: 3, minor: 2 }.minor_stride(), 1);
    }

    #[test]
    fn test_axis_shape_size() {
        assert_eq!(AxisShape { major: 2, minor: 2 }.size(), 4);
        assert_eq!(AxisShape { major: 2, minor: 3 }.size(), 6);
        assert_eq!(AxisShape { major: 3, minor: 2 }.size(), 6);
        assert_eq!(AxisShape { major: 3, minor: 3 }.size(), 9);
    }

    #[test]
    fn test_axis_shape_transpose() {
        let mut shape = AxisShape { major: 2, minor: 3 };

        shape.transpose();
        assert_eq!(shape, AxisShape { major: 3, minor: 2 });

        shape.transpose();
        assert_eq!(shape, AxisShape { major: 2, minor: 3 });
    }

    #[test]
    fn test_axis_shape_interpret() {
        let shape = AxisShape { major: 2, minor: 3 };

        assert_eq!(shape.interpret_with(Order::RowMajor), Shape::new(2, 3));
        assert_eq!(shape.interpret_with(Order::ColMajor), Shape::new(3, 2));

        assert_eq!(shape.interpret_nrows_with(Order::RowMajor), 2);
        assert_eq!(shape.interpret_nrows_with(Order::ColMajor), 3);

        assert_eq!(shape.interpret_ncols_with(Order::RowMajor), 3);
        assert_eq!(shape.interpret_ncols_with(Order::ColMajor), 2);
    }
}

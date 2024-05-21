use super::order::Order;
use crate::error::{Error, Result};

/// Any type that implements this trait can be used as the `shape` argument
/// in the constructors of [`Matrix`].
///
/// # Examples
///
/// ```
/// use matreex::{Matrix, Shape};
///
/// let foo = Matrix::<u8>::new(Shape::new(2, 3));
/// let bar = Matrix::<u8>::new((2, 3));
/// let baz = Matrix::<u8>::new([2, 3]);
/// ```
///
/// [`Matrix`]: crate::matrix::Matrix
pub trait ShapeLike {
    /// Returns the number of rows.
    fn nrows(&self) -> usize;

    /// Returns the number of columns.
    fn ncols(&self) -> usize;

    /// Returns the size of the shape.
    ///
    /// # Errors
    ///
    /// - [`Error::SizeOverflow`] if size exceeds [`usize::MAX`].
    fn size(&self) -> Result<usize> {
        self.nrows()
            .checked_mul(self.ncols())
            .ok_or(Error::SizeOverflow)
    }
}

/// A structure that represents the shape of a [`Matrix`].
///
/// # Notes
///
/// You might prefer using `(usize, usize)` instead when constructing
/// matrices. Refer to [`ShapeLike`] for more information.
///
/// [`Matrix`]: crate::matrix::Matrix
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Shape {
    // Number of rows.
    pub nrows: usize,

    // Number of columns.
    pub ncols: usize,
}

impl Shape {
    /// Creates a new [`Shape`] instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::Shape;
    ///
    /// let shape = Shape::new(2, 3);
    /// ```
    pub fn new(nrows: usize, ncols: usize) -> Self {
        Self { nrows, ncols }
    }
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.nrows, self.ncols)
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
    #[allow(unused)]
    pub(super) fn build(major: usize, minor: usize) -> Result<Self> {
        major.checked_mul(minor).ok_or(Error::SizeOverflow)?;
        Ok(Self { major, minor })
    }

    pub(super) fn major(&self) -> usize {
        self.major
    }

    pub(super) fn minor(&self) -> usize {
        self.minor
    }

    pub(super) fn major_stride(&self) -> usize {
        self.minor
    }

    pub(super) const fn minor_stride(&self) -> usize {
        1
    }

    pub(super) fn size(&self) -> usize {
        self.major * self.minor
    }

    pub(super) fn transpose(&mut self) -> &mut Self {
        (self.major, self.minor) = (self.minor, self.major);
        self
    }

    pub(super) fn interpret_with(&self, order: Order) -> Shape {
        let (nrows, ncols) = match order {
            Order::RowMajor => (self.major, self.minor),
            Order::ColMajor => (self.minor, self.major),
        };
        Shape { nrows, ncols }
    }

    pub(super) fn interpret_nrows_with(&self, order: Order) -> usize {
        match order {
            Order::RowMajor => self.major,
            Order::ColMajor => self.minor,
        }
    }

    pub(super) fn interpret_ncols_with(&self, order: Order) -> usize {
        match order {
            Order::RowMajor => self.minor,
            Order::ColMajor => self.major,
        }
    }

    pub(super) fn from_shape_with_unchecked<S: ShapeLike>(shape: S, order: Order) -> Self {
        let (major, minor) = match order {
            Order::RowMajor => (shape.nrows(), shape.ncols()),
            Order::ColMajor => (shape.ncols(), shape.nrows()),
        };
        Self { major, minor }
    }

    pub(super) fn try_from_shape_with<S: ShapeLike>(shape: S, order: Order) -> Result<Self> {
        shape.size()?;
        Ok(Self::from_shape_with_unchecked(shape, order))
    }

    pub(super) fn from_shape_with<S: ShapeLike>(shape: S, order: Order) -> Self {
        match Self::try_from_shape_with(shape, order) {
            Err(error) => panic!("{error}"),
            Ok(shape) => shape,
        }
    }
}

pub(super) trait IntoAxisShape {
    fn into_axis_shape_unchecked(self, order: Order) -> AxisShape;

    fn try_into_axis_shape(self, order: Order) -> Result<AxisShape>;

    fn into_axis_shape(self, order: Order) -> AxisShape;
}

impl<S: ShapeLike> IntoAxisShape for S {
    fn into_axis_shape_unchecked(self, order: Order) -> AxisShape {
        AxisShape::from_shape_with_unchecked(self, order)
    }

    fn try_into_axis_shape(self, order: Order) -> Result<AxisShape> {
        AxisShape::try_from_shape_with(self, order)
    }

    fn into_axis_shape(self, order: Order) -> AxisShape {
        AxisShape::from_shape_with(self, order)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_shape_like() {
        assert_eq!(Shape::new(2, 3).nrows(), 2);
        assert_eq!(Shape::new(2, 3).ncols(), 3);
        assert_eq!(Shape::new(2, 3).size(), Ok(6));
        assert_eq!(Shape::new(2, usize::MAX).size(), Err(Error::SizeOverflow));

        assert_eq!((2, 3).nrows(), 2);
        assert_eq!((2, 3).ncols(), 3);
        assert_eq!((2, 3).size(), Ok(6));
        assert_eq!((2, usize::MAX).size(), Err(Error::SizeOverflow));

        assert_eq!([2, 3].nrows(), 2);
        assert_eq!([2, 3].ncols(), 3);
        assert_eq!([2, 3].size(), Ok(6));
        assert_eq!([2, usize::MAX].size(), Err(Error::SizeOverflow));
    }

    #[test]
    fn test_struct_shape_new() {
        let expected = Shape { nrows: 2, ncols: 3 };

        assert_eq!(Shape::new(2, 3), expected);
        assert_ne!(Shape::new(3, 2), expected);
    }

    #[test]
    fn test_struct_shape_display() {
        assert_eq!(Shape::new(2, 3).to_string(), "(2, 3)");
        assert_eq!(Shape::new(3, 2).to_string(), "(3, 2)");
    }
}

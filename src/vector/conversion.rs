use super::kind::Kind;
use super::Vector; // super kind of me
use crate::error::{Error, Result};
use crate::matrix::shape::Shape;
use crate::matrix::Matrix;

impl<T> Vector<T> {
    /// Creates a new [`Vector`] instance from a [`Vec`].
    ///
    /// # Notes
    ///
    /// This method offers better performance compared to [`Vec::into`].
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::Vector;
    ///
    /// let vector = Vector::from_vec(vec![0, 1, 2]);
    /// assert_eq!(vector[0], 0);
    /// assert_eq!(vector[1], 1);
    /// assert_eq!(vector[2], 2);
    /// ```
    pub fn from_vec(src: Vec<T>) -> Self {
        Self {
            data: src,
            kind: Kind::default(),
        }
    }
}

impl<T> TryFrom<Matrix<T>> for Vector<T> {
    type Error = Error;

    fn try_from(value: Matrix<T>) -> Result<Self> {
        match value.shape() {
            Shape { nrows: 1, .. } => {
                let data = value.into_iter_elements().collect();
                let kind = Kind::RowVector;
                Ok(Self { data, kind })
            }
            Shape { ncols: 1, .. } => {
                let data = value.into_iter_elements().collect();
                let kind = Kind::ColVector;
                Ok(Self { data, kind })
            }
            _ => Err(Error::ShapeInconformable),
        }
    }
}

impl<T, S> From<S> for Vector<T>
where
    T: Clone,
    S: AsRef<[T]>,
{
    fn from(value: S) -> Self {
        Self {
            data: value.as_ref().to_vec(),
            kind: Kind::default(),
        }
    }
}

impl<T> From<Vector<T>> for Vec<T> {
    fn from(value: Vector<T>) -> Self {
        value.data
    }
}

impl<T: Default> std::iter::IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T: Default> std::iter::FromIterator<T> for Vector<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            data: iter.into_iter().collect(),
            kind: Kind::default(),
        }
    }
}

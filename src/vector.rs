//! This module defines [`Vector`] and all its related components.

pub mod kind;

mod fmt;

use self::kind::Kind;

/// [`Vector`] means vector.
///
/// To create a new [`Vector`] instance, you can use the [`vector!`] macro,
/// which works basically the same as the [`vec!`] macro does.
///
/// ```
/// use matreex::vector;
///
/// let vector = vector![0, 1, 2];
/// ```
///
/// [`vector!`]: crate::vector!
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Vector<T> {
    data: Vec<T>,
    kind: Kind,
}

impl<T> Vector<T> {
    /// Returns the kind of the vector.
    pub fn kind(&self) -> Kind {
        self.kind
    }
}

impl<T> Vector<T> {
    /// Transposes the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use matreex::vector;
    /// use matreex::vector::kind::Kind;
    ///
    /// let mut vector = vector![0, 1, 2];
    /// assert_eq!(vector.kind(), Kind::RowVector);
    ///
    /// vector.transpose();
    ///
    /// assert_eq!(vector.kind(), Kind::ColVector);
    /// ```
    pub fn transpose(&mut self) -> &mut Self {
        self.kind = !self.kind;
        self
    }
}

impl<T> std::ops::Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> std::ops::DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
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

impl<T: Default> std::iter::IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

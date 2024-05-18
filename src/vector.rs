pub mod kind;

mod fmt;

use self::kind::Kind;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Vector<T> {
    data: Vec<T>,
    kind: Kind,
}

impl<T> Vector<T> {
    pub fn transpose(&mut self) -> &mut Self {
        self.kind = !self.kind;
        self
    }
}

impl<T, S> From<S> for Vector<T>
where
    S: Into<Vec<T>>,
{
    fn from(value: S) -> Self {
        Self {
            data: value.into(),
            kind: Kind::default(),
        }
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

impl<T: Default> std::iter::IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

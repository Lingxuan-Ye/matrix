mod constructor;
mod dimension;
mod method;
mod ops;

use crate::shape::Shape;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix<T> {
    shape: Shape,
    data: Vec<T>,
}

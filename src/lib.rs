pub mod error;
pub mod shape;

pub use shape::Shape;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix<T> {
    shape: Shape,
    data: Vec<T>,
}

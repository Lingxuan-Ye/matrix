pub mod index;
pub mod order;
pub mod shape;

mod constructor;
mod internal;
mod method;
mod property;

use order::Order;
use shape::AxisShape;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Matrix<T> {
    data: Vec<T>,
    order: Order,
    shape: AxisShape,
}

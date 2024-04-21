mod constructor;
mod dimension;
mod internal;
mod method;
mod ops;

use crate::layout::MemoryLayout;
use dimension::Dimension;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Matrix<T> {
    data: Vec<T>,
    layout: MemoryLayout,
    dimension: Dimension,
}

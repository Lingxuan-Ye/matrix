use super::Matrix;
use crate::error::{Error, Result};
use crate::shape::Shape;

impl<T> Matrix<T> {
    pub(super) fn check_size(shape: &Shape) -> Result<usize> {
        let size = shape.size();
        // see more info at https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.with_capacity
        if std::mem::size_of::<T>() != 0 && size > isize::MAX as usize {
            Err(Error::CapacityExceeded)
        } else {
            Ok(size)
        }
    }
}

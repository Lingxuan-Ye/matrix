use super::Matrix;
use crate::error::{Error, Result};

impl<T> Matrix<T> {
    pub(super) fn check_size(size: usize) -> Result<usize> {
        // see more info at https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.with_capacity
        if std::mem::size_of::<T>() != 0 && size > isize::MAX as usize {
            Err(Error::SizeOverflow)
        } else {
            Ok(size)
        }
    }
}

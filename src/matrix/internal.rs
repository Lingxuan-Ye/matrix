use super::Matrix;
use crate::error::{Error, Result};

impl<T> Matrix<T> {
    pub(crate) fn check_size(size: usize) -> Result<usize> {
        // see more info at https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.with_capacity
        if std::mem::size_of::<T>() != 0 && size > isize::MAX as usize {
            Err(Error::CapacityExceeded)
        } else {
            Ok(size)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_size() {
        assert!(Matrix::<u8>::check_size(isize::MAX as usize).is_ok());
        assert_eq!(
            Matrix::<u8>::check_size(isize::MAX as usize + 1),
            Err(Error::CapacityExceeded)
        );
        assert!(Matrix::<()>::check_size(isize::MAX as usize + 1).is_ok());
    }
}

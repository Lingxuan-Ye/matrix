//! This module serves for error handling.

/// Enumerates errors for matrix operation failures.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// Error when matrix size exceeds [`usize::MAX`], which is, in fact
    /// pointless, since a matrix can store at most [`isize::MAX`] bytes.
    SizeOverflow,

    /// Error when attempting to reshape matrix to a different size.
    SizeMismatch,

    /// Error when total bytes exceeds [`isize::MAX`]. Refer to
    /// [`Vec::with_capacity`] for more information.
    CapacityExceeded,

    /// Error for accessing an index out of bounds. Not exposed in current
    /// public APIs.
    IndexOutOfBounds,

    /// Error for shape mismatch in arithmetic operations between matrices.
    MatricesInconformable,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Self::SizeOverflow => "size overflows",
            Self::SizeMismatch => "size does not match",
            Self::CapacityExceeded => "capacity exceeds",
            Self::IndexOutOfBounds => "index out of bounds",
            Self::MatricesInconformable => "matrices not conformable",
        };
        write!(f, "{content}")
    }
}

impl std::error::Error for Error {}

/// An alias for [`core::result::Result`].
pub type Result<T> = core::result::Result<T, Error>;

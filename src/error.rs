#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    SizeOverflow,
    SizeMismatch,
    CapacityExceeded,
    IndexOutOfBounds,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Self::SizeOverflow => "size overflows",
            Self::SizeMismatch => "size does not match",
            Self::CapacityExceeded => "capacity exceeds",
            Self::IndexOutOfBounds => "index out of bounds",
        };
        write!(f, "{content}")
    }
}

impl std::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;

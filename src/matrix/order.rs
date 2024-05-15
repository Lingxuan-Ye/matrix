/// Represents the memory order of a matrix.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Order {
    RowMajor,
    ColMajor,
}

impl std::ops::Not for Order {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::RowMajor => Self::ColMajor,
            Self::ColMajor => Self::RowMajor,
        }
    }
}

impl Default for Order {
    fn default() -> Self {
        Self::RowMajor
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_not() {
        assert_eq!(!Order::RowMajor, Order::ColMajor);
        assert_eq!(!Order::ColMajor, Order::RowMajor);
    }
}

/// Represents the memory order of a matrix.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Order {
    #[default]
    RowMajor,
    ColMajor,
}

impl Order {
    pub fn switch(self) -> Self {
        match self {
            Self::RowMajor => Self::ColMajor,
            Self::ColMajor => Self::RowMajor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch() {
        assert_eq!(Order::RowMajor.switch(), Order::ColMajor);
        assert_eq!(Order::ColMajor.switch(), Order::RowMajor);
    }
}

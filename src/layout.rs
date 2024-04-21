#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MemoryLayout {
    RowMajor,
    ColMajor,
}

impl std::ops::Not for MemoryLayout {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::RowMajor => Self::ColMajor,
            Self::ColMajor => Self::RowMajor,
        }
    }
}

impl Default for MemoryLayout {
    fn default() -> Self {
        Self::RowMajor
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_not() {
        assert_eq!(!MemoryLayout::RowMajor, MemoryLayout::ColMajor);
        assert_eq!(!MemoryLayout::ColMajor, MemoryLayout::RowMajor);
    }
}

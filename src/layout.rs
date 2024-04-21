#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MemoryLayout {
    RowMajor,
    ColMajor,
}

impl std::ops::Not for MemoryLayout {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            MemoryLayout::RowMajor => MemoryLayout::ColMajor,
            MemoryLayout::ColMajor => MemoryLayout::RowMajor,
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

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

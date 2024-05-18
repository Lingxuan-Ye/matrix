#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    RowVector,
    ColVector,
}

impl std::ops::Not for Kind {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::RowVector => Self::ColVector,
            Self::ColVector => Self::RowVector,
        }
    }
}

impl Default for Kind {
    fn default() -> Self {
        Self::RowVector
    }
}

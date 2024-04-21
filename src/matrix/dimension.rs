use crate::layout::MemoryLayout;
use crate::shape::Shape;

#[derive(Clone, Debug)]
pub(super) struct Dimension {
    major: usize,
    minor: usize,
}

impl Dimension {
    pub fn major(&self) -> usize {
        self.major
    }

    pub fn minor(&self) -> usize {
        self.minor
    }
}

impl Dimension {
    pub fn from_shape(shape: Shape, layout: MemoryLayout) -> Self {
        let (major, minor) = match layout {
            MemoryLayout::RowMajor => (shape.nrows(), shape.ncols()),
            MemoryLayout::ColMajor => (shape.ncols(), shape.nrows()),
        };
        Self { major, minor }
    }

    pub fn to_shape(&self, layout: MemoryLayout) -> Shape {
        let (nrows, ncols) = match layout {
            MemoryLayout::RowMajor => (self.major, self.minor),
            MemoryLayout::ColMajor => (self.minor, self.major),
        };
        Shape::build(nrows, ncols).expect("this will never fail")
    }
}

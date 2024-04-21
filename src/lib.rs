mod error;
mod index;
mod layout;
mod matrix;
mod shape;

pub use error::{Error, Result};
pub use index::Index;
pub use layout::MemoryLayout;
pub use matrix::Matrix;
pub use shape::{Shape, TryIntoShape};

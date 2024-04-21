mod error;
mod index;
mod matrix;
mod shape;

pub use error::{Error, Result};
pub use index::Index;
pub use matrix::Matrix;
pub use shape::{Shape, TryIntoShape};

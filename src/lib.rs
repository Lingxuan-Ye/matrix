pub mod error;
pub mod matrix;

mod macros;

pub use error::{Error, Result};
pub use matrix::index::Index;
pub use matrix::order::Order;
pub use matrix::shape::{Shape, TryIntoShape};
pub use matrix::Matrix;

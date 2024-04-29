pub mod error;
pub mod matrix;

mod macros;

pub use self::error::{Error, Result};
pub use self::matrix::index::{Index, IndexLike};
pub use self::matrix::iter::{MatrixIter, VectorIter};
pub use self::matrix::order::Order;
pub use self::matrix::shape::{Shape, ShapeLike};
pub use self::matrix::Matrix;

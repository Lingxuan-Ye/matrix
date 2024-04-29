//! A simple matrix implementation.
//!
//! # Quick Start
//!
//! ```
//! use matreex::matrix;
//!
//! let lhs = matrix![[0, 1, 2], [3, 4, 5]];
//! let rhs = matrix![[0, 1], [2, 3], [4, 5]];
//!
//! assert_eq!(lhs * rhs, matrix![[10, 13], [28, 40]]);
//! ```
//!
//! # FAQs
//!
//! ## Why `matreex` instead of `matrix`?
//!
//! Hmm ... Who knows? Could be a name conflict.
//!
//! ## Where are the docs?
//!
//! Coming soon (maybe not that soon).

pub mod error;
pub mod matrix;

mod macros;

pub use self::error::{Error, Result};
pub use self::matrix::index::{Index, IndexLike};
pub use self::matrix::iter::{MatrixIter, VectorIter};
pub use self::matrix::order::Order;
pub use self::matrix::shape::{Shape, ShapeLike};
pub use self::matrix::Matrix;

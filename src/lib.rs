//! A simple matrix implementation.
//!
//! # Quick Start
//!
//! ## Addition
//!
//! ```
//! use matreex::matrix;
//!
//! let lhs = matrix![[0, 1, 2], [3, 4, 5]];
//! let rhs = matrix![[5, 4, 3], [2, 1, 0]];
//!
//! assert_eq!(lhs + rhs, matrix![[5, 5, 5], [5, 5, 5]]);
//! ```
//!
//! ## Subtraction
//!
//! ```
//! use matreex::matrix;
//!
//! let lhs = matrix![[0, 1, 2], [3, 4, 5]];
//! let rhs = matrix![[5, 4, 3], [2, 1, 0]];
//!
//! assert_eq!(lhs - rhs, matrix![[-5, -3, -1], [1, 3, 5]]);
//! ```
//!
//! ## Multiplication
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

pub mod error;
pub mod marker;
pub mod matrix;
pub mod vector;

mod fmt;
mod macros;

pub use self::error::{Error, Result};
pub use self::marker::{Number, Scalar};
pub use self::matrix::index::Index;
pub use self::matrix::iter::{MatrixIter, VectorIter};
pub use self::matrix::order::Order;
pub use self::matrix::shape::Shape;
pub use self::matrix::Matrix;
pub use self::vector::Vector;

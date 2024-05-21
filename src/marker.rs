//! This module provides marker traits for scalar and number types.

use crate::vector::Vector;

/// A marker trait for scalar types.
///
/// # Notes
///
/// A [`Scalar`] does not have to be a scalar in the mathematical sense;
/// it can be implemented for any type that represents a finite nesting
/// of other types, including any concrete [`Matrix`] (though you cannot
/// actually do this because of the orphan rule). However, because the
/// generic version of [`Matrix`] can nest itself infinitely, marking it
/// would result in a compile-time overflow evaluation error due to the
/// arithmetic operations implementation. For simplicity, no [`Matrix`]
/// has been marked with this trait.
///
/// [`Matrix`]: crate::matrix::Matrix
pub trait Scalar {}

impl Scalar for u8 {}
impl Scalar for u16 {}
impl Scalar for u32 {}
impl Scalar for u64 {}
impl Scalar for u128 {}
impl Scalar for usize {}
impl Scalar for i8 {}
impl Scalar for i16 {}
impl Scalar for i32 {}
impl Scalar for i64 {}
impl Scalar for i128 {}
impl Scalar for isize {}
impl Scalar for f32 {}
impl Scalar for f64 {}
impl Scalar for bool {}
impl Scalar for char {}
impl Scalar for str {}
impl Scalar for () {}
impl<T> Scalar for [T] {}
impl<T, const N: usize> Scalar for [T; N] {}
impl<T> Scalar for Vec<T> {}
impl<T> Scalar for Vector<T> {}
impl<T> Scalar for *const T {}
impl<T> Scalar for *mut T {}
impl<T> Scalar for Box<T> {}
impl<T: Scalar> Scalar for &T {}

/// A marker trait for number types. It is implemented for all the primitive
/// numeric types.
pub trait Number: Scalar {}

impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for u128 {}
impl Number for usize {}
impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for i128 {}
impl Number for isize {}
impl Number for f32 {}
impl Number for f64 {}

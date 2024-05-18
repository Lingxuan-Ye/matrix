/// Creates a new [`Matrix`] instance from literal.
///
/// # Examples
///
/// ```
/// use matreex::matrix;
///
/// let matrix = matrix![[0, 1, 2], [3, 4, 5]];
/// ```
///
/// [`Matrix`]: crate::matrix::Matrix
#[macro_export]
macro_rules! matrix {
    [$($col:expr),+ $(,)?] => {
        $crate::matrix::Matrix::from_2darray(std::boxed::Box::new([$($col,)+]))
    };
}

/// Creates a new [`Vector`] instance from literal.
///
/// # Examples
///
/// ```
/// use matreex::vector;
///
/// let vector = vector![0, 1, 2];
/// ```
///
/// [`Vector`]: crate::vector::Vector
#[macro_export]
macro_rules! vector {
    [] => {
        $crate::vector::Vector::from(vec![])
    };

    [$elem:expr; $n:expr] => {
        $crate::vector::Vector::from(vec![$elem; $n])
    };

    [$($x:expr),+ $(,)?] => {
        $crate::vector::Vector::from(vec![$($x),+])
    };
}

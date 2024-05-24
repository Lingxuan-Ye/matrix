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
        $crate::matrix::Matrix::from(std::boxed::Box::new([$($col,)+]))
    };
}

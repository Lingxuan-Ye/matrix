/// Creates a new [`Matrix`] instance from literal.
///
/// # Examples
///
/// ```
/// use matreex::{matrix, Matrix};
///
/// let foo: Matrix<i32> = matrix![];
/// let bar = matrix![[0; 3]; 2];
/// let baz = matrix![[0, 1, 2], [3, 4, 5]];
/// ```
///
/// [`Matrix`]: crate::matrix::Matrix
#[macro_export]
macro_rules! matrix {
    [] => {{
        let shape = $crate::matrix::shape::Shape::new(0, 0);
        $crate::matrix::Matrix::new(shape)
    }};

    [$elem:expr; $n:expr] => {
        $crate::matrix::Matrix::from([$elem; $n])
    };

    [$($col:expr),+ $(,)?] => {
        $crate::matrix::Matrix::from([$($col,)+])
    };
}

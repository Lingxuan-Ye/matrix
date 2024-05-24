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

/// Creates a new row vector from literal.
///
/// # Examples
///
/// ```
/// use matreex::{matrix, row_vec, Matrix};
///
/// let foo: Matrix<i32> = row_vec![];
/// assert_eq!(foo.nrows(), 1);
/// assert_eq!(foo.ncols(), 0);
///
/// let bar = row_vec![0; 3];
/// assert_eq!(bar, matrix![[0, 0, 0]]);
///
/// let baz = row_vec![0, 1, 2];
/// assert_eq!(baz, matrix![[0, 1, 2]]);
/// ```
#[macro_export]
macro_rules! row_vec {
    [] => {{
        let shape = $crate::matrix::shape::Shape::new(1, 0);
        $crate::matrix::Matrix::new(shape)
    }};

    [$elem:expr; $n:expr] => {
        $crate::matrix::Matrix::from([[$elem; $n]])
    };

    [$($x:expr),+ $(,)?] => {
        $crate::matrix::Matrix::from([[$($x),+]])
    };
}

/// Creates a new column vector from literal.
///
/// # Examples
///
/// ```
/// use matreex::{matrix, col_vec, Matrix};
///
/// let foo: Matrix<i32> = col_vec![];
/// assert_eq!(foo.nrows(), 0);
/// assert_eq!(foo.ncols(), 1);
///
/// let bar = col_vec![0; 3];
/// assert_eq!(bar, matrix![[0], [0], [0]]);
///
/// let baz = col_vec![0, 1, 2];
/// assert_eq!(baz, matrix![[0], [1], [2]]);
/// ```
#[macro_export]
macro_rules! col_vec {
    [] => {{
        let shape = $crate::matrix::shape::Shape::new(0, 1);
        $crate::matrix::Matrix::new(shape)
    }};

    [$elem:expr; $n:expr] => {
        $crate::matrix::Matrix::from([[$elem]; $n])
    };

    [$($x:expr),+ $(,)?] => {
        $crate::matrix::Matrix::from([$([$x]),+])
    };
}

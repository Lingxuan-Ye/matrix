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
        $crate::matrix::Matrix::empty()
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
        $crate::matrix::Matrix::from([[]])
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
        let mut matrix = $crate::matrix::Matrix::from([[]]);
        matrix.transpose();
        matrix
    }};

    [$elem:expr; $n:expr] => {
        $crate::matrix::Matrix::from([[$elem]; $n])
    };

    [$($x:expr),+ $(,)?] => {
        $crate::matrix::Matrix::from([$([$x]),+])
    };
}

// For simplicity, all arithmetic operations rely on the behavior of `$t`,
// including those performed on references.

/// Implements scalar addition for [`Matrix`].
///
/// # Notes
///
/// A `scalar` does not have to be a scalar in the mathematical sense. Instead,
/// it can be any type except for [`Matrix`]. However, if you do need to treat
/// some concrete type of [`Matrix`] as a scalar, you can wrap it in a newtype
/// and implement all the necessary trait bounds for it.
///
/// [`Matrix`]: crate::matrix::Matrix
#[macro_export]
macro_rules! impl_scalar_add {
    ($($t:ty)*) => {
        $(
            impl std::ops::Add<$t> for $crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn add(self, rhs: $t) -> Self::Output {
                    self.scalar_operation_consume_self(&rhs, |element, scalar| element + scalar.clone())
                }
            }

            impl std::ops::Add<&$t> for $crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn add(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation_consume_self(rhs, |element, scalar| element + scalar.clone())
                }
            }

            impl std::ops::Add<$t> for &$crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn add(self, rhs: $t) -> Self::Output {
                    self.scalar_operation(&rhs, |element, scalar| element.clone() + scalar.clone())
                }
            }

            impl std::ops::Add<&$t> for &$crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn add(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation(rhs, |element, scalar| element.clone() + scalar.clone())
                }
            }

            impl std::ops::Add<$t> for $crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn add(self, rhs: $t) -> Self::Output {
                    self.scalar_operation_consume_self(&rhs, |element, scalar| element.clone() + scalar.clone())
                }
            }

            impl std::ops::Add<&$t> for $crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn add(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation_consume_self(rhs, |element, scalar| element.clone() + scalar.clone())
                }
            }

            impl std::ops::Add<$t> for &$crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn add(self, rhs: $t) -> Self::Output {
                    self.scalar_operation(&rhs, |element, scalar| (*element).clone() + scalar.clone())
                }
            }

            impl std::ops::Add<&$t> for &$crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn add(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation(rhs, |element, scalar| (*element).clone() + scalar.clone())
                }
            }

            impl std::ops::Add<$crate::matrix::Matrix<$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn add(self, rhs: $crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(&self, |element, scalar| scalar.clone() + element)
                }
            }

            impl std::ops::Add<$crate::matrix::Matrix<$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn add(self, rhs: $crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(self, |element, scalar| scalar.clone() + element)
                }
            }

            impl std::ops::Add<&$crate::matrix::Matrix<$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn add(self, rhs: &$crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation(&self, |element, scalar| scalar.clone() + element.clone())
                }
            }

            impl std::ops::Add<&$crate::matrix::Matrix<$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn add(self, rhs: &$crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation(self, |element, scalar| scalar.clone() + element.clone())
                }
            }

            impl std::ops::Add<$crate::matrix::Matrix<&$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn add(self, rhs: $crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(&self, |element, scalar| scalar.clone() + element.clone())
                }
            }

            impl std::ops::Add<$crate::matrix::Matrix<&$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn add(self, rhs: $crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(self, |element, scalar| scalar.clone() + element.clone())
                }
            }

            impl std::ops::Add<&$crate::matrix::Matrix<&$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn add(self, rhs: &$crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation(&self, |element, scalar| scalar.clone() + (*element).clone())
                }
            }

            impl std::ops::Add<&$crate::matrix::Matrix<&$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn add(self, rhs: &$crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation(self, |element, scalar| scalar.clone() + (*element).clone())
                }
            }

            impl std::ops::AddAssign<$t> for $crate::matrix::Matrix<$t> {
                fn add_assign(&mut self, rhs: $t) {
                    self.scalar_operation_assign(&rhs, |element, scalar| *element += scalar.clone());
                }
            }

            impl std::ops::AddAssign<&$t> for $crate::matrix::Matrix<$t> {
                fn add_assign(&mut self, rhs: &$t) {
                    self.scalar_operation_assign(rhs, |element, scalar| *element += scalar.clone());
                }
            }

            impl std::ops::AddAssign<$t> for $crate::matrix::Matrix<&mut $t> {
                fn add_assign(&mut self, rhs: $t) {
                    self.scalar_operation_assign(&rhs, |element, scalar| **element += scalar.clone());
                }
            }

            impl std::ops::AddAssign<&$t> for $crate::matrix::Matrix<&mut $t> {
                fn add_assign(&mut self, rhs: &$t) {
                    self.scalar_operation_assign(rhs, |element, scalar| **element += scalar.clone());
                }
            }
        )*
    }
}

/// Implements scalar subtraction for [`Matrix`].
///
/// # Notes
///
/// Refer to [`impl_scalar_add!`] for more information.
///
/// [`Matrix`]: crate::matrix::Matrix
#[macro_export]
macro_rules! impl_scalar_sub {
    ($($t:ty)*) => {
        $(
            impl std::ops::Sub<$t> for $crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: $t) -> Self::Output {
                    self.scalar_operation_consume_self(&rhs, |element, scalar| element - scalar.clone())
                }
            }

            impl std::ops::Sub<&$t> for $crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation_consume_self(rhs, |element, scalar| element - scalar.clone())
                }
            }

            impl std::ops::Sub<$t> for &$crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: $t) -> Self::Output {
                    self.scalar_operation(&rhs, |element, scalar| element.clone() - scalar.clone())
                }
            }

            impl std::ops::Sub<&$t> for &$crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation(rhs, |element, scalar| element.clone() - scalar.clone())
                }
            }

            impl std::ops::Sub<$t> for $crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: $t) -> Self::Output {
                    self.scalar_operation_consume_self(&rhs, |element, scalar| element.clone() - scalar.clone())
                }
            }

            impl std::ops::Sub<&$t> for $crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation_consume_self(rhs, |element, scalar| element.clone() - scalar.clone())
                }
            }

            impl std::ops::Sub<$t> for &$crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: $t) -> Self::Output {
                    self.scalar_operation(&rhs, |element, scalar| (*element).clone() - scalar.clone())
                }
            }

            impl std::ops::Sub<&$t> for &$crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation(rhs, |element, scalar| (*element).clone() - scalar.clone())
                }
            }

            impl std::ops::Sub<$crate::matrix::Matrix<$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: $crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(&self, |element, scalar| scalar.clone() - element)
                }
            }

            impl std::ops::Sub<$crate::matrix::Matrix<$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: $crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(self, |element, scalar| scalar.clone() - element)
                }
            }

            impl std::ops::Sub<&$crate::matrix::Matrix<$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: &$crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation(&self, |element, scalar| scalar.clone() - element.clone())
                }
            }

            impl std::ops::Sub<&$crate::matrix::Matrix<$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: &$crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation(self, |element, scalar| scalar.clone() - element.clone())
                }
            }

            impl std::ops::Sub<$crate::matrix::Matrix<&$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: $crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(&self, |element, scalar| scalar.clone() - element.clone())
                }
            }

            impl std::ops::Sub<$crate::matrix::Matrix<&$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: $crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(self, |element, scalar| scalar.clone() - element.clone())
                }
            }

            impl std::ops::Sub<&$crate::matrix::Matrix<&$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: &$crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation(&self, |element, scalar| scalar.clone() - (*element).clone())
                }
            }

            impl std::ops::Sub<&$crate::matrix::Matrix<&$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn sub(self, rhs: &$crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation(self, |element, scalar| scalar.clone() - (*element).clone())
                }
            }

            impl std::ops::SubAssign<$t> for $crate::matrix::Matrix<$t> {
                fn sub_assign(&mut self, rhs: $t) {
                    self.scalar_operation_assign(&rhs, |element, scalar| *element -= scalar.clone());
                }
            }

            impl std::ops::SubAssign<&$t> for $crate::matrix::Matrix<$t> {
                fn sub_assign(&mut self, rhs: &$t) {
                    self.scalar_operation_assign(rhs, |element, scalar| *element -= scalar.clone());
                }
            }

            impl std::ops::SubAssign<$t> for $crate::matrix::Matrix<&mut $t> {
                fn sub_assign(&mut self, rhs: $t) {
                    self.scalar_operation_assign(&rhs, |element, scalar| **element -= scalar.clone());
                }
            }

            impl std::ops::SubAssign<&$t> for $crate::matrix::Matrix<&mut $t> {
                fn sub_assign(&mut self, rhs: &$t) {
                    self.scalar_operation_assign(rhs, |element, scalar| **element -= scalar.clone());
                }
            }
        )*
    }
}

/// Implements scalar multiplication for [`Matrix`].
///
/// # Notes
///
/// Refer to [`impl_scalar_add!`] for more information.
///
/// [`Matrix`]: crate::matrix::Matrix
#[macro_export]
macro_rules! impl_scalar_mul {
    ($($t:ty)*) => {
        $(
            impl std::ops::Mul<$t> for $crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn mul(self, rhs: $t) -> Self::Output {
                    self.scalar_operation_consume_self(&rhs, |element, scalar| element * scalar.clone())
                }
            }

            impl std::ops::Mul<&$t> for $crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn mul(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation_consume_self(rhs, |element, scalar| element * scalar.clone())
                }
            }

            impl std::ops::Mul<$t> for &$crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn mul(self, rhs: $t) -> Self::Output {
                    self.scalar_operation(&rhs, |element, scalar| element.clone() * scalar.clone())
                }
            }

            impl std::ops::Mul<&$t> for &$crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn mul(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation(rhs, |element, scalar| element.clone() * scalar.clone())
                }
            }

            impl std::ops::Mul<$t> for $crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn mul(self, rhs: $t) -> Self::Output {
                    self.scalar_operation_consume_self(&rhs, |element, scalar| element.clone() * scalar.clone())
                }
            }

            impl std::ops::Mul<&$t> for $crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn mul(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation_consume_self(rhs, |element, scalar| element.clone() * scalar.clone())
                }
            }

            impl std::ops::Mul<$t> for &$crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn mul(self, rhs: $t) -> Self::Output {
                    self.scalar_operation(&rhs, |element, scalar| (*element).clone() * scalar.clone())
                }
            }

            impl std::ops::Mul<&$t> for &$crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn mul(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation(rhs, |element, scalar| (*element).clone() * scalar.clone())
                }
            }

            impl std::ops::Mul<$crate::matrix::Matrix<$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn mul(self, rhs: $crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(&self, |element, scalar| scalar.clone() * element)
                }
            }

            impl std::ops::Mul<$crate::matrix::Matrix<$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn mul(self, rhs: $crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(self, |element, scalar| scalar.clone() * element)
                }
            }

            impl std::ops::Mul<&$crate::matrix::Matrix<$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn mul(self, rhs: &$crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation(&self, |element, scalar| scalar.clone() * element.clone())
                }
            }

            impl std::ops::Mul<&$crate::matrix::Matrix<$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn mul(self, rhs: &$crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation(self, |element, scalar| scalar.clone() * element.clone())
                }
            }

            impl std::ops::Mul<$crate::matrix::Matrix<&$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn mul(self, rhs: $crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(&self, |element, scalar| scalar.clone() * element.clone())
                }
            }

            impl std::ops::Mul<$crate::matrix::Matrix<&$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn mul(self, rhs: $crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(self, |element, scalar| scalar.clone() * element.clone())
                }
            }

            impl std::ops::Mul<&$crate::matrix::Matrix<&$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn mul(self, rhs: &$crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation(&self, |element, scalar| scalar.clone() * (*element).clone())
                }
            }

            impl std::ops::Mul<&$crate::matrix::Matrix<&$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn mul(self, rhs: &$crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation(self, |element, scalar| scalar.clone() * (*element).clone())
                }
            }

            impl std::ops::MulAssign<$t> for $crate::matrix::Matrix<$t> {
                fn mul_assign(&mut self, rhs: $t) {
                    self.scalar_operation_assign(&rhs, |element, scalar| *element *= scalar.clone());
                }
            }

            impl std::ops::MulAssign<&$t> for $crate::matrix::Matrix<$t> {
                fn mul_assign(&mut self, rhs: &$t) {
                    self.scalar_operation_assign(rhs, |element, scalar| *element *= scalar.clone());
                }
            }

            impl std::ops::MulAssign<$t> for $crate::matrix::Matrix<&mut $t> {
                fn mul_assign(&mut self, rhs: $t) {
                    self.scalar_operation_assign(&rhs, |element, scalar| **element *= scalar.clone());
                }
            }

            impl std::ops::MulAssign<&$t> for $crate::matrix::Matrix<&mut $t> {
                fn mul_assign(&mut self, rhs: &$t) {
                    self.scalar_operation_assign(rhs, |element, scalar| **element *= scalar.clone());
                }
            }
        )*
    }
}

/// Implements scalar division for [`Matrix`].
///
/// # Notes
///
/// Refer to [`impl_scalar_add!`] for more information.
///
/// [`Matrix`]: crate::matrix::Matrix
#[macro_export]
macro_rules! impl_scalar_div {
    ($($t:ty)*) => {
        $(
            impl std::ops::Div<$t> for $crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn div(self, rhs: $t) -> Self::Output {
                    self.scalar_operation_consume_self(&rhs, |element, scalar| element / scalar.clone())
                }
            }

            impl std::ops::Div<&$t> for $crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn div(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation_consume_self(rhs, |element, scalar| element / scalar.clone())
                }
            }

            impl std::ops::Div<$t> for &$crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn div(self, rhs: $t) -> Self::Output {
                    self.scalar_operation(&rhs, |element, scalar| element.clone() / scalar.clone())
                }
            }

            impl std::ops::Div<&$t> for &$crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn div(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation(rhs, |element, scalar| element.clone() / scalar.clone())
                }
            }

            impl std::ops::Div<$t> for $crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn div(self, rhs: $t) -> Self::Output {
                    self.scalar_operation_consume_self(&rhs, |element, scalar| element.clone() / scalar.clone())
                }
            }

            impl std::ops::Div<&$t> for $crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn div(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation_consume_self(rhs, |element, scalar| element.clone() / scalar.clone())
                }
            }

            impl std::ops::Div<$t> for &$crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn div(self, rhs: $t) -> Self::Output {
                    self.scalar_operation(&rhs, |element, scalar| (*element).clone() / scalar.clone())
                }
            }

            impl std::ops::Div<&$t> for &$crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn div(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation(rhs, |element, scalar| (*element).clone() / scalar.clone())
                }
            }

            impl std::ops::Div<$crate::matrix::Matrix<$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn div(self, rhs: $crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(&self, |element, scalar| scalar.clone() / element)
                }
            }

            impl std::ops::Div<$crate::matrix::Matrix<$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn div(self, rhs: $crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(self, |element, scalar| scalar.clone() / element)
                }
            }

            impl std::ops::Div<&$crate::matrix::Matrix<$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn div(self, rhs: &$crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation(&self, |element, scalar| scalar.clone() / element.clone())
                }
            }

            impl std::ops::Div<&$crate::matrix::Matrix<$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn div(self, rhs: &$crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation(self, |element, scalar| scalar.clone() / element.clone())
                }
            }

            impl std::ops::Div<$crate::matrix::Matrix<&$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn div(self, rhs: $crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(&self, |element, scalar| scalar.clone() / element.clone())
                }
            }

            impl std::ops::Div<$crate::matrix::Matrix<&$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn div(self, rhs: $crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(self, |element, scalar| scalar.clone() / element.clone())
                }
            }

            impl std::ops::Div<&$crate::matrix::Matrix<&$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn div(self, rhs: &$crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation(&self, |element, scalar| scalar.clone() / (*element).clone())
                }
            }

            impl std::ops::Div<&$crate::matrix::Matrix<&$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn div(self, rhs: &$crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation(self, |element, scalar| scalar.clone() / (*element).clone())
                }
            }

            impl std::ops::DivAssign<$t> for $crate::matrix::Matrix<$t> {
                fn div_assign(&mut self, rhs: $t) {
                    self.scalar_operation_assign(&rhs, |element, scalar| *element /= scalar.clone());
                }
            }

            impl std::ops::DivAssign<&$t> for $crate::matrix::Matrix<$t> {
                fn div_assign(&mut self, rhs: &$t) {
                    self.scalar_operation_assign(rhs, |element, scalar| *element /= scalar.clone());
                }
            }

            impl std::ops::DivAssign<$t> for $crate::matrix::Matrix<&mut $t> {
                fn div_assign(&mut self, rhs: $t) {
                    self.scalar_operation_assign(&rhs, |element, scalar| **element /= scalar.clone());
                }
            }

            impl std::ops::DivAssign<&$t> for $crate::matrix::Matrix<&mut $t> {
                fn div_assign(&mut self, rhs: &$t) {
                    self.scalar_operation_assign(rhs, |element, scalar| **element /= scalar.clone());
                }
            }
        )*
    }
}

/// Implements scalar remainder operation for [`Matrix`].
///
/// # Notes
///
/// Refer to [`impl_scalar_add!`] for more information.
///
/// [`Matrix`]: crate::matrix::Matrix
#[macro_export]
macro_rules! impl_scalar_rem {
    ($($t:ty)*) => {
        $(
            impl std::ops::Rem<$t> for $crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn rem(self, rhs: $t) -> Self::Output {
                    self.scalar_operation_consume_self(&rhs, |element, scalar| element % scalar.clone())
                }
            }

            impl std::ops::Rem<&$t> for $crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn rem(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation_consume_self(rhs, |element, scalar| element % scalar.clone())
                }
            }

            impl std::ops::Rem<$t> for &$crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn rem(self, rhs: $t) -> Self::Output {
                    self.scalar_operation(&rhs, |element, scalar| element.clone() % scalar.clone())
                }
            }

            impl std::ops::Rem<&$t> for &$crate::matrix::Matrix<$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn rem(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation(rhs, |element, scalar| element.clone() % scalar.clone())
                }
            }

            impl std::ops::Rem<$t> for $crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn rem(self, rhs: $t) -> Self::Output {
                    self.scalar_operation_consume_self(&rhs, |element, scalar| element.clone() % scalar.clone())
                }
            }

            impl std::ops::Rem<&$t> for $crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn rem(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation_consume_self(rhs, |element, scalar| element.clone() % scalar.clone())
                }
            }

            impl std::ops::Rem<$t> for &$crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn rem(self, rhs: $t) -> Self::Output {
                    self.scalar_operation(&rhs, |element, scalar| (*element).clone() % scalar.clone())
                }
            }

            impl std::ops::Rem<&$t> for &$crate::matrix::Matrix<&$t> {
                type Output = $crate::matrix::Matrix<$t>;

                fn rem(self, rhs: &$t) -> Self::Output {
                    self.scalar_operation(rhs, |element, scalar| (*element).clone() % scalar.clone())
                }
            }

            impl std::ops::Rem<$crate::matrix::Matrix<$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn rem(self, rhs: $crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(&self, |element, scalar| scalar.clone() % element)
                }
            }

            impl std::ops::Rem<$crate::matrix::Matrix<$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn rem(self, rhs: $crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(self, |element, scalar| scalar.clone() % element)
                }
            }

            impl std::ops::Rem<&$crate::matrix::Matrix<$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn rem(self, rhs: &$crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation(&self, |element, scalar| scalar.clone() % element.clone())
                }
            }

            impl std::ops::Rem<&$crate::matrix::Matrix<$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn rem(self, rhs: &$crate::matrix::Matrix<$t>) -> Self::Output {
                    rhs.scalar_operation(self, |element, scalar| scalar.clone() % element.clone())
                }
            }

            impl std::ops::Rem<$crate::matrix::Matrix<&$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn rem(self, rhs: $crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(&self, |element, scalar| scalar.clone() % element.clone())
                }
            }

            impl std::ops::Rem<$crate::matrix::Matrix<&$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn rem(self, rhs: $crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation_consume_self(self, |element, scalar| scalar.clone() % element.clone())
                }
            }

            impl std::ops::Rem<&$crate::matrix::Matrix<&$t>> for $t {
                type Output = $crate::matrix::Matrix<$t>;

                fn rem(self, rhs: &$crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation(&self, |element, scalar| scalar.clone() % (*element).clone())
                }
            }

            impl std::ops::Rem<&$crate::matrix::Matrix<&$t>> for &$t {
                type Output = $crate::matrix::Matrix<$t>;

                fn rem(self, rhs: &$crate::matrix::Matrix<&$t>) -> Self::Output {
                    rhs.scalar_operation(self, |element, scalar| scalar.clone() % (*element).clone())
                }
            }

            impl std::ops::RemAssign<$t> for $crate::matrix::Matrix<$t> {
                fn rem_assign(&mut self, rhs: $t) {
                    self.scalar_operation_assign(&rhs, |element, scalar| *element %= scalar.clone());
                }
            }

            impl std::ops::RemAssign<&$t> for $crate::matrix::Matrix<$t> {
                fn rem_assign(&mut self, rhs: &$t) {
                    self.scalar_operation_assign(rhs, |element, scalar| *element %= scalar.clone());
                }
            }

            impl std::ops::RemAssign<$t> for $crate::matrix::Matrix<&mut $t> {
                fn rem_assign(&mut self, rhs: $t) {
                    self.scalar_operation_assign(&rhs, |element, scalar| **element %= scalar.clone());
                }
            }

            impl std::ops::RemAssign<&$t> for $crate::matrix::Matrix<&mut $t> {
                fn rem_assign(&mut self, rhs: &$t) {
                    self.scalar_operation_assign(rhs, |element, scalar| **element %= scalar.clone());
                }
            }
        )*
    }
}

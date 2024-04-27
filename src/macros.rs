#[macro_export]
macro_rules! matrix {
    [ $($col:expr),+ $(,)? ] => {
        $crate::matrix::Matrix::from_2darray(std::boxed::Box::new([$($col,)+]))
    };
}

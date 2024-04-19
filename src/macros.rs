#[macro_export]
macro_rules! matrix {
    [ $($x:expr),+ $(,)? ] => {
        $crate::Matrix::from_2darray(std::boxed::Box::new([$($x,)+]))
    };
}
